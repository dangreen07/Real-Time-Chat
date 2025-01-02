use diesel::result::Error;
use diesel::PgConnection;
use diesel::prelude::*;

use crate::models::User;
use crate::{models::Contact, schema::users, schema::contacts};

pub fn get_contacts(conn: &mut PgConnection, arg_user_id: &uuid::Uuid) -> Result<Vec<User>, &'static str> {
    let contacts_list: Result<Vec<(Contact, User)>, Error> = contacts::table
            .inner_join(users::table.on(users::id.eq(contacts::contact_id)))
            .select((Contact::as_select(), User::as_select()))
            .filter(contacts::user_id.eq(arg_user_id))
            .load::<(Contact, User)>(conn);
    
    let contacts_list = match contacts_list {
        Ok(contacts_list) => contacts_list,
        Err(_) => return Err("Error getting contacts")
    };
    let result = contacts_list.iter().map(|x| x.1.clone()).collect::<Vec<User>>();
    return Ok(result);
}