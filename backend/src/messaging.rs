mod contact;
mod messages;

use actix_web::{error::BlockingError, get, web, HttpRequest, HttpResponse, Responder};
use chrono::DateTime;
use contact::get_contacts;
use messages::{get_messages, store_message};
use real_time_chat_backend::types::{ResponseMessage, UserResponse};
use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use actix_ws::Message;

use crate::{get_user, models::NewMessage, AppState};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSocketMessage {
    pub session_id: uuid::Uuid,
    pub message: String,
    pub recipient: uuid::Uuid,
    pub sent_at: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketObject {
    pub session_id: uuid::Uuid,
    pub object_type: String,
    pub object: String
}

#[get("/ws")]
pub async fn websocket(req: HttpRequest, body: web::Payload, app_state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
    
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                    println!("Ping received");
                }
                Message::Text(msg) => {
                    let result = serde_json::from_str::<WebSocketObject>(&msg);
                    let result = match result {
                        Ok(result) => result,
                        Err(_) => return
                    };
                    if result.object_type == "identify" {
                        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");
                        let user = get_user(&mut conn, &result.session_id);
                        let user = match user {
                            Ok(user) => user,
                            Err(_) => return
                        };
                        {
                            app_state.websocket_sessions.lock().unwrap().insert(user.id, session.clone());
                        }
                    }
                    else if result.object_type == "message" {
                        let message_object = serde_json::from_str::<WebSocketMessage>(&result.object);
                        let message_object = match message_object {
                            Ok(message) => message,
                            Err(_) => return
                        };
                        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");
                        let user = get_user(&mut conn, &result.session_id);
                        let user = match user {
                            Ok(user) => user,
                            Err(_) => return
                        };
                        {
                            // Send the message to the recipient with the websocket session
                            let websocket_sessions = app_state.websocket_sessions.lock().expect("Could not get websocket sessions lock");
                            let session = websocket_sessions.get(&message_object.recipient);
                            let sent_time = DateTime::from_timestamp_millis(message_object.sent_at);
                            let sent_time = match sent_time {
                                Some(sent_time) => sent_time,
                                None => return
                            };
                            let sent_time = sent_time.naive_utc();
                            let message_object = NewMessage {
                                recipient_id: &message_object.recipient,
                                message: &message_object.message,
                                sent_at: sent_time,
                                user_id: &user.id
                            };
                            let message_object = serde_json::to_string(&message_object).expect("Could not serialize message object");
                            if let Some(session) = session {
                                let mut session = session.clone();
                                let _ = session.text(message_object).await;
                            }
                        }
                        // Store the message in the database
                        let sent_time = DateTime::from_timestamp_millis(message_object.sent_at);
                        let sent_time = match sent_time {
                            Some(sent_time) => sent_time,
                            None => return
                        };
                        let sent_time = sent_time.naive_utc();
                        let new_message = NewMessage {
                            user_id: &user.id,
                            recipient_id: &message_object.recipient,
                            message: &message_object.message,
                            sent_at: sent_time
                        };
                        let _ = store_message(&mut conn, new_message);
                    }
                },
                _ => break,
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

#[get("/messages/{recipient_id}/{session_id}")]
pub async fn get_messages_endpoint(session_data: web::Path<(uuid::Uuid, uuid::Uuid)>, app_state: web::Data<AppState>) -> impl Responder {
    let (recipient_id, session_id) = session_data.into_inner();
    let block_result = web::block(move || {
        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");
        let user = get_user(&mut conn, &session_id);
        let user = match user {
            Ok(user) => user,
            Err(_) => return Err("Error getting user")
        };
        let messages = get_messages(&mut conn, &user.id, &recipient_id);
        let messages = match messages {
            Ok(messages) => messages,
            Err(_) => return Err("Error getting messages")
        };
        return Ok(messages);
    }).await;
    let block_result = match block_result {
        Ok(block_result) => block_result,
        Err(_) => return HttpResponse::InternalServerError().body("Error during async operation")
    };
    let messages = match block_result {
        Ok(messages) => messages,
        Err(_) => return HttpResponse::InternalServerError().body("Error during database operation")
    };
    let messages = messages.iter().map(|x| ResponseMessage {
        id: x.id,
        user_id: x.user_id,
        recipient_id: x.recipient_id,
        message: x.message.clone(),
        sent_at: x.sent_at.and_utc().timestamp_millis()
    }).collect::<Vec<ResponseMessage>>();
    HttpResponse::Ok().json(messages)
}

#[get("/contacts/{session_id}")]
pub async fn get_contacts_endpoint(session_data: web::Path::<uuid::Uuid>, app_state: web::Data<AppState>) -> impl Responder {
    let session_id = session_data.into_inner();
    let block_result: Result<Result<Vec<UserResponse>, &str>, BlockingError> = web::block(move || {
        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");

        // Get the user
        let user = get_user(&mut conn, &session_id);
        let user = match user {
            Ok(user) => user,
            Err(_) => return Err("Error getting user")
        };
        let user_id = user.id;

        // Gets the contacts list
        let contacts_list = get_contacts(&mut conn, &user_id);
        let contacts_list = match contacts_list {
            Ok(contacts_list) => contacts_list,
            Err(_) => return Err("Error getting contacts")
        };
        let contacts_list = contacts_list.iter().map(|x| UserResponse {
            id: x.id,
            username: x.username.clone(),
            full_name: x.full_name.clone(),
            permissions: x.permission.clone()
        }).collect::<Vec<UserResponse>>();
        Ok(contacts_list)
    }).await;
    let block_result = match block_result {
        Ok(block_result) => block_result,
        Err(_) => return HttpResponse::InternalServerError().body("Error during database operation")
    };
    let contacts_list = match block_result {
        Ok(contacts_list) => contacts_list,
        Err(_) => return HttpResponse::InternalServerError().body("Error during database operation")
    };
    HttpResponse::Ok().json(contacts_list)
}