use chrono::Duration;
use diesel::result::Error;
use diesel::PgConnection;
use diesel::prelude::*;
use password_hash::{
    rand_core::OsRng,
    SaltString
};
use argon2::{
    Argon2,
    PasswordHasher
};
use uuid::Uuid;

use crate::models::*;

pub fn create_user(conn: &mut PgConnection, arg_username: &str, arg_password: &str) -> Result<User, &'static str> {
    use crate::schema::users;
    use crate::schema::users::dsl::*;

    let user = users.filter(username.eq(arg_username)).first::<User>(conn);
    match user {
        Ok(_) => return Err("User already exists"),
        Err(_) => ()
    };

    // Hashing the password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hashed = argon2.hash_password(arg_password.as_bytes(), &salt);
    let password_hashed = match password_hashed {
        Ok(password_hashed) => password_hashed.to_string(),
        Err(_) => return Err("Error hashing password")
    };

    // Creating the user
    let new_user = NewUser {
        username: arg_username,
        password_hash: &password_hashed,
    };

    // Inserting the user into the database
    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn);
    let result = match result {
        Ok(result) => result,
        Err(_) => return Err("Error creating user")
    };
    Ok(result)
}

pub fn create_session(conn: &mut PgConnection, arg_user_id: &Uuid) -> Result<Session, &'static str> {
    use crate::schema::sessions;

    let new_session = NewSession {
        user_id: arg_user_id,
        expiry:  chrono::Utc::now().naive_utc() + Duration::days(30),
    };

    let result = diesel::insert_into(sessions::table)
        .values(&new_session)
        .returning(Session::as_returning())
        .get_result(conn);
    match result {
        Ok(result) => return Ok(result),
        Err(_) => return Err("Error creating session")
    };
}

pub fn valid_session(conn: &mut PgConnection, arg_session_id: &uuid::Uuid) -> bool {
    use crate::schema::sessions::dsl::*;

    let response = sessions.filter(id.eq(arg_session_id)).select(Session::as_select()).first::<Session>(conn);
    let session = match response {
        Ok(session) => session,
        Err(_) => return false
    };

    return session.expiry >= chrono::Utc::now().naive_utc();
}

pub fn invalidate_session(conn: &mut PgConnection, arg_session_id: &uuid::Uuid) -> Result<usize, Error> {
    use crate::schema::sessions::dsl::*;

    diesel::delete(sessions.filter(id.eq(arg_session_id))).execute(conn)
}

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

pub fn verify_user(conn: &mut PgConnection, arg_username: &str, arg_password: &str) -> Result<Uuid, &'static str> {
    use crate::schema::users::dsl::*;

    let response = users.filter(username.eq(arg_username)).select(User::as_select()).first::<User>(conn);
    let user = match response {
        Ok(user) => user,
        Err(_) => return Err("Username does not exist!")
    };
    
    let encoded_hash = user.password_hash;
    let vals = encoded_hash.split("$").collect::<Vec<&str>>();
    let salt = SaltString::from_b64(vals[4]).expect("Failed to decode salt");
    let argon2 = Argon2::default();
    let password_hashed = argon2.hash_password(arg_password.as_bytes(), &salt).expect("Failed to hash password!").to_string();
    if password_hashed == encoded_hash {
        return Ok(user.id);
    }
    return Err("Username or password is incorrect!");
}