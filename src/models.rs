use diesel::prelude::*;
use super::schema::users;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub createdAt: String,
    pub modifiedAt: String,
    pub privilege: String,
}
