mod auth;
mod messaging;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use auth::*;
use messaging::{get_contacts_endpoint, get_messages_endpoint, websocket};
use real_time_chat_backend::{get_connection_pool, types::DbPool};

pub mod models;
pub mod schema;

#[derive(Clone)]
struct AppState {
    websocket_sessions: Arc<Mutex<HashMap<uuid::Uuid, actix_ws::Session>>>,
    postgres_connection_pool: DbPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection_pool = get_connection_pool();
    // let redis_connection_pool = get_redis_connection_pool();

    let app_state = AppState {
        websocket_sessions: Arc::new(Mutex::new(HashMap::new())),
        postgres_connection_pool: connection_pool.clone()
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Note: This is insecure and should not be used in production
            .allowed_headers(vec!["Content-Type"])
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(connection_pool.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .service(signup)
            .service(login)
            .service(logout)
            .service(validate_session)
            .service(users_info)
            .service(websocket)
            .service(get_contacts_endpoint)
            .service(get_messages_endpoint)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}