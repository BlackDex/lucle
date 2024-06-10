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
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel::{Connection, QueryDsl, RunQueryDsl};

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
    match LucleDBConnection::establish(database_url).unwrap() {
        LucleDBConnection::Pg(conn) => insert_user(
            LucleDBConnection::Pg(conn),
            username,
            password_hash,
            email,
            "admin".to_string(),
        )?,
        LucleDBConnection::Sqlite(conn) => insert_user(
            LucleDBConnection::Sqlite(conn),
            username,
            password_hash,
            email,
            "admin".to_string(),
        )?,
        LucleDBConnection::Mysql(conn) => insert_user(
            LucleDBConnection::Mysql(conn),
            username,
            password_hash,
            email,
            "admin".to_string(),
        )?,
    }

    Ok(())
}

pub fn login(database_url: &str, username: String, password: String) -> DatabaseResult<()> {
    match LucleDBConnection::establish(database_url).unwrap() {
        LucleDBConnection::Pg(conn) => get_user(LucleDBConnection::Pg(conn), username, password)?,
        LucleDBConnection::Sqlite(conn) => {
            get_user(LucleDBConnection::Sqlite(conn), username, password)?
        }
        LucleDBConnection::Mysql(conn) => {
            get_user(LucleDBConnection::Mysql(conn), username, password)?
        }
    }
    Ok(())
}

pub fn is_default_user(database_url: &str) -> bool {
    match LucleDBConnection::establish(database_url).unwrap() {
        LucleDBConnection::Pg(conn) => table_size(LucleDBConnection::Pg(conn)),
        LucleDBConnection::Sqlite(conn) => table_size(LucleDBConnection::Sqlite(conn)),
        LucleDBConnection::Mysql(conn) => table_size(LucleDBConnection::Mysql(conn)),
    }
}

pub fn reset_password(database_url: &str, email: String) -> DatabaseResult<()> {
    match LucleDBConnection::establish(database_url).unwrap() {
        LucleDBConnection::Pg(conn) => lost_password(LucleDBConnection::Pg(conn), email),
        LucleDBConnection::Sqlite(conn) => lost_password(LucleDBConnection::Sqlite(conn), email),
        LucleDBConnection::Mysql(conn) => lost_password(LucleDBConnection::Mysql(conn), email),
    }
}
fn insert_user(
    mut conn: LucleDBConnection,
    username: String,
    password_hash: String,
    email: String,
    role: String,
) -> DatabaseResult<()> {
    let new_user = NewUser {
        username: username,
        password: password_hash,
        email: email,
        role: role,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)?;
    Ok(())
}

fn get_user(mut conn: LucleDBConnection, username: String, password: String) -> DatabaseResult<()> {
    let user = users::table
        .filter(users::dsl::username.eq(username))
        .select(User::as_select())
        .first(&mut conn)
        .optional();
    match user {
        Ok(Some(val)) => {
            let parsed_hash = PasswordHash::new(&val.password).unwrap();
            Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
            if val.role == "admin" {
                Ok(())
            } else {
                Err(DatabaseError::NotAuthorized)
            }
        }
        Ok(None) => Err(DatabaseError::UserNotFound),
        Err(err) => Err(DatabaseError::QueryError(err)),
    }
}

fn table_size(mut conn: LucleDBConnection) -> bool {
    let user = users::table.count().get_result::<i64>(&mut conn);
    if let Ok(count) = user {
        count > 0
    } else {
        false
    }
}

fn lost_password(mut conn: LucleDBConnection, email: String) -> DatabaseResult<()> {
    match users::table
        .filter(users::dsl::email.eq(email))
        .select(User::as_select())
        .first(&mut conn)
        .optional()
    {
        Ok(Some(val)) => {
            let token = utils::generate_jwt(val.username, val.email.clone());
            if diesel::update(users::table.filter(users::dsl::email.eq(val.email.clone())))
                .set(users::dsl::reset_token.eq(token))
                .returning(User::as_returning())
                .get_result(&mut conn)
                .is_ok()
            {
                utils::send_mail(&email, &val.email, "test", "hi");
            }
        }
        Ok(None) => Err("Unknow email".to_string()),
        Err(err) => Err("Connection failed :".to_string() + &err.to_string()),
    }

    Ok(())
}
