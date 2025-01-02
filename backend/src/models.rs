use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::{sessions, users};

#[derive(Queryable, Selectable, Serialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub full_name: String,
    pub password_hash: String,
    pub permission: String
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub full_name: &'a str,
    pub password_hash: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub expiry: NaiveDateTime,
    pub user_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession<'a> {
    pub user_id: &'a uuid::Uuid,
    pub expiry: NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = crate::schema::contacts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contact {
    pub id: Uuid,
    pub user_id: Uuid,
    pub contact_id: Uuid
}

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recipient_id: Uuid,
    pub message: String,
    pub sent_at: NaiveDateTime
}

#[derive(Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::messages)]
pub struct NewMessage<'a> {
    pub user_id: &'a uuid::Uuid,
    pub recipient_id: &'a uuid::Uuid,
    pub message: &'a str,
    pub sent_at: NaiveDateTime
}