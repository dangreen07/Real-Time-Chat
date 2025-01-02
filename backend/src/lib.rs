use diesel::{prelude::*, r2d2::Pool};
use dotenvy::dotenv;
use redis::Client;
use types::DbPool;
use std::env;

pub mod types;

pub fn get_connection_pool() -> DbPool {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn get_redis_connection_pool() -> Pool<Client> {
    dotenv().ok();
    
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url.as_str()).expect("Could not connect to Redis");
    let pool = Pool::builder().build(client).expect("Could not build Redis connection pool");
    return pool;
}