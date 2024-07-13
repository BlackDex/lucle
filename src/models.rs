use super::schema::{
    repositories, sql_types::UsersRepositoriesPermissionEnum, users, users_repositories,
};
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::FromSqlRow;
use diesel::{
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    AsExpression,
};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub reset_token: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = repositories)]
pub struct Repository {
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users_repositories)]
pub struct UsersRepositories {
    pub user_id: i32,
    pub repository_name: String,
    pub permission: Permission,
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = UsersRepositoriesPermissionEnum)]
pub enum Permission {
    Write = 1,
    Read = 2,
}

impl<DB> ToSql<UsersRepositoriesPermissionEnum, DB> for Permission
where
    DB: Backend,
    i32: ToSql<UsersRepositoriesPermissionEnum, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        match self {
            Permission::Write => 1.to_sql(out),
            Permission::Read => 2.to_sql(out),
        }
    }
}

impl<DB> FromSql<Text, DB> for Permission
where
    DB: Backend,
    i32: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Permission::Write),
            2 => Ok(Permission::Read),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}
