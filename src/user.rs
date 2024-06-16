use crate::database::LucleDBConnection;
use crate::database_errors::{DatabaseError, DatabaseResult};
use crate::models::{NewUser, User};
use crate::schema::users;
use crate::utils;
use argon2::{
    self,
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel::{select, Connection, QueryDsl, RunQueryDsl};

pub fn create_user(
    database_url: &str,
    username: String,
    password: String,
    email: String,
) -> DatabaseResult<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    let mut conn = LucleDBConnection::establish(database_url)?;
    let now = select(diesel::dsl::now)
        .get_result::<NaiveDateTime>(&mut conn)
        .unwrap();

    let new_user = NewUser {
        username,
        password: password_hash,
        email,
        created_at: now,
        modified_at: now,
        role: "admin".to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)?;
    Ok(())
}

pub fn login(database_url: &str, username: String, password: String) -> DatabaseResult<String> {
    let mut conn = LucleDBConnection::establish(database_url)?;
    let user = users::table
        .filter(users::dsl::username.eq(username.clone()))
        .select(User::as_select())
        .first(&mut conn)
        .optional();
    match user {
        Ok(Some(val)) => {
            let parsed_hash = PasswordHash::new(&val.password).unwrap();
            Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
            if val.role == "admin" {
                let token = utils::generate_jwt(username, val.email);
                Ok(token)
            } else {
                Err(DatabaseError::NotAuthorized)
            }
        }
        Ok(None) => Err(DatabaseError::UserNotFound),
        Err(err) => Err(DatabaseError::QueryError(err)),
    }
}

pub fn is_table_and_user_created(database_url: &str) -> DatabaseResult<()> {
    let mut conn = LucleDBConnection::establish(database_url)?;
    match users::table.count().get_result::<i64>(&mut conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(DatabaseError::QueryError(err)),
    }
}

pub fn reset_password(database_url: &str, email: String) -> DatabaseResult<()> {
    let mut conn = LucleDBConnection::establish(database_url)?;
    match users::table
        .filter(users::dsl::email.eq(email.clone()))
        .select(User::as_select())
        .first(&mut conn)
        .optional()
    {
        Ok(Some(val)) => {
            let token = utils::generate_jwt(val.username, val.email.clone());
            if diesel::update(users::table.filter(users::dsl::email.eq(val.email.clone())))
                .set(users::dsl::reset_token.eq(token))
                .execute(&mut conn)
                .is_ok()
            {
                utils::send_mail(&email, &val.email, "test", "hi");
                return Ok(());
            }
        }
        Ok(None) => return Err(DatabaseError::EmailNotFound),
        Err(err) => return Err(DatabaseError::QueryError(err)),
    }

    Ok(())
}
