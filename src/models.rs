use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}
