use crate::errors::Error;
use crate::models::{NewUser, Permission, Repository, User, UsersRepositories};
use crate::schema::{repositories, users, users_repositories};
use crate::utils;
use argon2::{
    self,
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::select;
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
use diesel_async::{AsyncMysqlConnection, RunQueryDsl};
use once_cell::sync::Lazy;

pub struct LucleUser {
    pub username: String,
    pub token: String,
}

static POOL: Lazy<Pool<AsyncMysqlConnection>> = Lazy::new(|| {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncMysqlConnection>::new(
        "mysql://root:swp@localhost/lucle",
    );
    Pool::builder(config).build().unwrap()
});

pub async fn create_user(username: String, password: String, email: String) -> Result<(), Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    let mut conn = POOL.get().await?;
    let now = select(diesel::dsl::now)
        .get_result::<NaiveDateTime>(&mut conn)
        .await?;

    let new_user = NewUser {
        username,
        password: password_hash,
        email,
        created_at: now,
        modified_at: now,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .await?;
    Ok(())
}

pub async fn register_update_server(username: String, repository: String) -> Result<(), Error> {
    let mut conn = POOL.get().await?;
    let now = select(diesel::dsl::now)
        .get_result::<NaiveDateTime>(&mut conn)
        .await?;

    let repo = Repository {
        name: repository,
        created_at: now,
    };

    diesel::insert_into(repositories::table)
        .values(&repo)
        .execute(&mut conn)
        .await?;

    match users::table
        .filter(users::dsl::username.eq(username))
        .select(User::as_select())
        .first(&mut conn)
        .await
    {
        Ok(val) => {
            let users_repos = UsersRepositories {
                user_id: val.id,
                repository_name: repository,
                permission: Permission::Write,
            };
            diesel::insert_into(users_repositories::table)
                .values(&users_repos)
                .execute(&mut conn)
                .await?;
            return Ok(());
        }
        Err(err) => Err(crate::errors::Error::QueryError(err)),
    };
    Ok(())
}

pub async fn login(username_or_email: String, password: String) -> Result<LucleUser, Error> {
    let mut conn = POOL.get().await?;
    match users::table
        .filter(users::dsl::username.eq(username_or_email.clone()))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .optional()
    {
        Ok(Some(val)) => login_user(val.username, val.password, password, val.email),
        Ok(None) => {
            match users::table
                .filter(users::dsl::email.eq(username_or_email.clone()))
                .select(User::as_select())
                .first(&mut conn)
                .await
                .optional()
            {
                Ok(Some(val)) => login_user(val.username, val.password, password, val.email),
                Ok(None) => Err(crate::errors::Error::UserNotFound),
                Err(err) => Err(crate::errors::Error::QueryError(err)),
            }
        }
        Err(err) => Err(crate::errors::Error::QueryError(err)),
    }
}

pub async fn is_table_and_user_created() -> Result<(), Error> {
    let mut conn = POOL.get().await?;
    match users::table.count().get_result::<i64>(&mut conn).await {
        Ok(_) => Ok(()),
        Err(err) => Err(crate::errors::Error::QueryError(err)),
    }
}

pub async fn reset_password(email: String) -> Result<(), Error> {
    let mut conn = POOL.get().await?;
    match users::table
        .filter(users::dsl::email.eq(email.clone()))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .optional()
    {
        Ok(Some(val)) => {
            let token = utils::generate_jwt(val.username, val.email.clone());
            if diesel::update(users::table.filter(users::dsl::email.eq(val.email.clone())))
                .set(users::dsl::reset_token.eq(token))
                .execute(&mut conn)
                .await
                .is_ok()
            {
                //                utils::send_mail(&email, &val.email, "test", "hi");
                return Ok(());
            }
        }
        Ok(None) => return Err(crate::errors::Error::EmailNotFound),
        Err(err) => return Err(crate::errors::Error::QueryError(err)),
    }

    Ok(())
}

fn login_user(
    username: String,
    stored_password: String,
    password: String,
    email: String,
) -> Result<LucleUser, Error> {
    let parsed_hash = PasswordHash::new(&stored_password).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
    let token = utils::generate_jwt(username.clone(), email);
    Ok(LucleUser {
        username: username,
        token: token,
    })
}
