use crate::database::{handle_error, Backend};
use crate::database_errors::{DatabaseError, DatabaseResult};
use crate::models::{NewUser, Users};
use crate::schema::users;
use argon2::{self, password_hash::PasswordVerifier, Argon2, PasswordHash};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel::{
    select, Connection, MysqlConnection, PgConnection, QueryDsl, RunQueryDsl, SqliteConnection,
};
use std::path::Path;

pub fn create_user(
    database_url: &str,
    username: String,
    password: String,
    email: String,
) -> DatabaseResult<()> {
    match Backend::for_url(database_url) {
        Backend::Pg => {
            let conn = &mut PgConnection::establish(database_url).unwrap_or_else(handle_error);
            let now = select(diesel::dsl::now)
                .get_result::<NaiveDateTime>(conn)
                .unwrap();

            let new_user = NewUser {
                username: &username,
                password: &password,
                created_at: now,
                modified_at: now,
                email: &email,
                privilege: "admin",
            };

            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(conn)?;
        }
        Backend::Mysql => {
            let conn = &mut MysqlConnection::establish(database_url).unwrap_or_else(handle_error);
            let now = select(diesel::dsl::now)
                .get_result::<NaiveDateTime>(conn)
                .unwrap();

            let new_user = NewUser {
                username: &username,
                password: &password,
                created_at: now,
                modified_at: now,
                email: &email,
                privilege: "admin",
            };

            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(conn)?;
        }
        Backend::Sqlite => {
            let conn = &mut SqliteConnection::establish(database_url).unwrap_or_else(handle_error);
            let now = select(diesel::dsl::now)
                .get_result::<NaiveDateTime>(conn)
                .unwrap();

            let new_user = NewUser {
                username: &username,
                password: &password,
                created_at: now,
                modified_at: now,
                email: &email,
                privilege: "admin",
            };

            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(conn)?;
        }
    }

    Ok(())
}

pub fn login(database_url: &str, username: &str, password: &str) -> DatabaseResult<()> {
    let user;
    match Backend::for_url(database_url) {
        Backend::Pg => {
            let conn = &mut PgConnection::establish(database_url).unwrap_or_else(handle_error);
            user = users::table
                .filter(users::dsl::username.eq(username))
                .select(Users::as_select())
                .first(conn)
                .optional();
        }
        Backend::Mysql => {
            let conn = &mut MysqlConnection::establish(database_url).unwrap_or_else(handle_error);
            user = users::table
                .filter(users::dsl::username.eq(username))
                .select(Users::as_select())
                .first(conn)
                .optional();
        }
        Backend::Sqlite => {
            let conn = &mut SqliteConnection::establish(database_url).unwrap_or_else(handle_error);
            user = users::table
                .filter(users::dsl::username.eq(username))
                .select(Users::as_select())
                .first(conn)
                .optional();
        }
    }
    match user {
        Ok(Some(val)) => {
            let parsed_hash = PasswordHash::new(&val.password).unwrap();
            Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
            if val.privilege == "admin" {
                return Ok(());
            } else {
                return Err(DatabaseError::NotAuthorized);
            }
        }
        Ok(None) => return Err(DatabaseError::UserNotFound),
        Err(err) => return Err(DatabaseError::QueryError(err)),
    }
}

pub fn is_default_user(database_url: &str) -> bool {
    match Backend::for_url(database_url) {
        Backend::Pg => {
            let conn = &mut PgConnection::establish(database_url).unwrap_or_else(handle_error);
            let user = users::table.count().get_result::<i64>(conn);
            if let Ok(count) = user {
                count > 0
            } else {
                false
            }
        }
        Backend::Mysql => {
            let conn = &mut MysqlConnection::establish(database_url).unwrap_or_else(handle_error);
            let user = users::table.count().get_result::<i64>(conn);
            if let Ok(count) = user {
                count > 0
            } else {
                false
            }
        }
        Backend::Sqlite => {
            if Path::new("lucle.db").exists() {
                let conn =
                    &mut SqliteConnection::establish(database_url).unwrap_or_else(handle_error);
                let user = users::table.count().get_result::<i64>(conn);
                if let Ok(count) = user {
                    count > 0
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}
