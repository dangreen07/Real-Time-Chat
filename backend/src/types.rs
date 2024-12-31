use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PostedUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SessionReturn {
    pub session_id: uuid::Uuid,
    pub error: String
}

#[derive(Serialize, Deserialize)]
pub struct SessionInput {
    pub session_id: uuid::Uuid
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub permissions: String
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;