mod database;

use actix_web::{get, post, web, HttpResponse, Responder};
use database::{create_session, create_user, invalidate_session, valid_session, verify_user};
use diesel::prelude::*;
use real_time_chat_backend::types::{PostedUser, SessionInput, SessionReturn, SignupUser, UserResponse};

use crate::{models::{Session, User}, AppState};

pub fn get_user(conn: &mut PgConnection, arg_session_id: &uuid::Uuid) -> Result<User, &'static str> {
    use crate::schema::{sessions, users};

    let resp = sessions::table
        .inner_join(users::table)
        .filter(sessions::id.eq(arg_session_id))
        .select((Session::as_select(), User::as_select()))
        .first::<(Session, User)>(conn);

    let resp = match resp {
        Ok(session) => session,
        Err(_) => return Err("Error getting user")
    };
    let user = resp.1;
    Ok(user)
}

#[post("/signup")]
pub async fn signup(user: web::Json<SignupUser>, app_state: web::Data<AppState>) -> impl Responder {
    if user.username.len() < 3 {
        let output = SessionReturn {
            session_id: uuid::Uuid::nil(),
            error: "Username must be at least 3 characters long!".to_string()
        };
        return HttpResponse::InternalServerError().json(output);
    }
    if user.password.len() < 8 {
        let output = SessionReturn {
            session_id: uuid::Uuid::nil(),
            error: "Password must be at least 8 characters long!".to_string()
        };
        return HttpResponse::InternalServerError().json(output);
    }

    let block_result = web::block(move || {
        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");
        let user = create_user(&mut conn, &user.username, &user.full_name, &user.password);
        let user = match user {
            Ok(user) => user,
            Err(_) => return Err("Error creating user")
        };
        let session = create_session(&mut conn, &user.id);
        let session = match session {
            Ok(session) => session,
            Err(_) => return Err("Error creating session")
        };
        Ok(session)
    }).await;
    let block_result = match block_result {
        Ok(block_result) => block_result,
        Err(_) => {
            let output = SessionReturn {
                session_id: uuid::Uuid::nil(),
                error: "Error during database operation".to_string()
            };
            return HttpResponse::InternalServerError().json(output);
        }
    };
    let session = match block_result {
        Ok(session) => session,
        Err(_) => {
            let output = SessionReturn {
                session_id: uuid::Uuid::nil(),
                error: "Error during database operation".to_string()
            };
            return HttpResponse::InternalServerError().json(output);
        }
    };
    let output = SessionReturn {
        session_id: session.id,
        error: "".to_string()
    };

    HttpResponse::Ok().json(output)
}

#[post("/login")]
pub async fn login(user: web::Json<PostedUser>, app_state: web::Data<AppState>) -> impl Responder {
    if user.username.len() < 3 || user.password.len() < 8 {
        let output = SessionReturn {
            session_id: uuid::Uuid::nil(),
            error: "Username or password is incorrect!".to_string()
        };
        return HttpResponse::InternalServerError().json(output);
    }

    let block_result = web::block(move || {
        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");
        let user = verify_user(&mut conn, &user.username, &user.password);
        // If the user doesn't exist or the password is incorrect, return an error
        let user = match user {
            Ok(user) => user,
            Err(err) => return Err(err)
        };
        let session = create_session(&mut conn, &user);
        let session = match session {
            Ok(session) => session,
            Err(err) => return Err(err)
        };
        Ok(session)
    }).await;
    let block_result = match block_result {
        Ok(block_result) => block_result,
        Err(_) => {
            let output = SessionReturn {
                session_id: uuid::Uuid::nil(),
                error: "Error during database operation".to_string()
            };
            return HttpResponse::InternalServerError().json(output);
        }
    };
    let session = match block_result {
        Ok(session) => session,
        Err(err) => {
            let output = SessionReturn {
                session_id: uuid::Uuid::nil(),
                error: err.to_string()
            };
            return HttpResponse::InternalServerError().json(output);
        }
    };
    let output = SessionReturn {
        session_id: session.id,
        error: "".to_string()
    };

    HttpResponse::Ok().json(output)
}

#[post("/logout")]
pub async fn logout(session_data: web::Json<SessionInput>, app_state: web::Data<AppState>) -> impl Responder {
    let session_id = session_data.session_id;
    let block_result = web::block(move || {
        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");
        let session = invalidate_session(&mut conn, &session_id);
        let session = match session {
            Ok(session) => session,
            Err(_) => return Err("Error invalidating session")
        };
        Ok(session)
    }).await;
    let block_result = match block_result {
        Ok(block_result) => block_result,
        Err(_) => return HttpResponse::InternalServerError().body("Error during database operation")
    };
    match block_result {
        Ok(session) => session,
        Err(_) => return HttpResponse::InternalServerError().body("Error during database operation")
    };
    return HttpResponse::Ok().body("");
}

#[post("/validate_session")]
pub async fn validate_session(session_data: web::Json<SessionInput>, app_state: web::Data<AppState>) -> impl Responder {
    let session_id = session_data.session_id;
    let block_result = web::block(move || {
        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");
        let session = valid_session(&mut conn, &session_id);
        return session;
    }).await;
    let result = match block_result {
        Ok(block_result) => block_result,
        Err(_) => return HttpResponse::InternalServerError().body("Error during database operation")
    };
    if result {
        return HttpResponse::Ok().body("true");
    }
    else {
        return HttpResponse::Ok().body("false");
    }
}

#[get("/user/{session_id}")]
pub async fn users_info(session_data: web::Path<uuid::Uuid>, app_state: web::Data<AppState>) -> impl Responder {
    let session_id = session_data.into_inner();
    let block_result = web::block(move || {
        let mut conn = app_state.postgres_connection_pool.get().expect("Could not get connection from pool");
        let user = get_user(&mut conn, &session_id);
        let user = match user {
            Ok(user) => user,
            Err(_) => return Err("Error getting user")
        };
        Ok(user)
    }).await;
    let block_result = match block_result {
        Ok(block_result) => block_result,
        Err(_) => return HttpResponse::InternalServerError().body("Error during database operation")
    };
    let user = match block_result {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().body("Error during database operation")
    };
    let output = UserResponse {
        id: user.id,
        username: user.username,
        full_name: user.full_name,
        permissions: user.permission
    };
    return HttpResponse::Ok().json(output);
}