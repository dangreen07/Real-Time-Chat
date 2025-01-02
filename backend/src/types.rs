use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PostedUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupUser {
    pub username: String,
    pub full_name: String,
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

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub full_name: String,
    pub permissions: String
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
pub struct ResponseMessage {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub recipient_id: uuid::Uuid,
    pub message: String,
    pub sent_at: i64
}