use crate::database::{handle_error, Backend};
use crate::database_errors::DatabaseResult;
use crate::models::{NewUser, Users};
use crate::schema::users;
use std::path::Path;
//use self::schema::users::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel::{
    select, Connection, MysqlConnection, PgConnection, QueryDsl, RunQueryDsl, SqliteConnection,
};

pub fn create_user(database_url: &str, username: String, password: String) -> DatabaseResult<()> {
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
                email: "allo",
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
                email: "allo",
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
                email: "allo",
                privilege: "admin",
            };

            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(conn)?;
        }
    }

    Ok(())
}

pub fn login(database_url: &str, username: &str) -> DatabaseResult<()> {
    match Backend::for_url(database_url) {
        Backend::Pg => {
            let conn = &mut PgConnection::establish(database_url).unwrap_or_else(handle_error);
            let user = users::table
                .filter(users::dsl::username.eq(username))
                .select(Users::as_select())
                .first(conn)
                .optional();
        }
        Backend::Mysql => {
            let conn = &mut MysqlConnection::establish(database_url).unwrap_or_else(handle_error);
            let user = users::table
                .filter(users::dsl::username.eq(username))
                .select(Users::as_select())
                .first(conn)
                .optional();
        }
        Backend::Sqlite => {
            let conn = &mut SqliteConnection::establish(database_url).unwrap_or_else(handle_error);
            let user = users::table
                .filter(users::dsl::username.eq(username))
                .select(Users::as_select())
                .first(conn)
                .optional();
	println!("{:?}", user.unwrap());
        }
    }
  Ok(())
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
