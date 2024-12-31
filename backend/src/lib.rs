use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use dotenvy::dotenv;
use types::DbPool;
use std::env;

pub mod types;

pub fn get_connection_pool() -> DbPool {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}