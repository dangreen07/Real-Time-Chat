use diesel::PgConnection;
use diesel::prelude::*;

use crate::{models::{Message, NewMessage}, schema::messages};

/// Stores a message between a user and a recipient
pub fn store_message(conn: &mut PgConnection, message: NewMessage) -> bool {
    let result = diesel::insert_into(messages::table)
        .values(&message)
        .execute(conn);
    match result {
        Ok(_) => return true,
        Err(_) => return false
    };
}

/// Gets messages between a user and a recipient
pub fn get_messages(conn: &mut PgConnection, arg_user_id: &uuid::Uuid, arg_recipient_id: &uuid::Uuid) -> Result<Vec<Message>, &'static str> {
    use crate::schema::messages::dsl::*;

    // Here I am filtering the messages by user id and recipient id, these need to be interchangeable in the query
    let messages_list = messages.filter(
        (user_id.eq(arg_user_id).and(recipient_id.eq(arg_recipient_id)))
        .or(user_id.eq(arg_recipient_id).and(recipient_id.eq(arg_user_id)))
    ).select(Message::as_select()).load::<Message>(conn);
    let messages_list = match messages_list {
        Ok(messages_list) => messages_list,
        Err(_) => return Err("Error getting messages")
    };
    return Ok(messages_list);
}