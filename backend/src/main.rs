mod auth;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use auth::*;
use real_time_chat_backend::get_connection_pool;

pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection_pool = get_connection_pool();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Note: This is insecure and should not be used in production
            .allowed_headers(vec!["Content-Type"])
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(connection_pool.clone()))
            .service(signup)
            .service(login)
            .service(logout)
            .service(validate_session)
            .service(users_info)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}