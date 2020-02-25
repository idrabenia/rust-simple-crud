use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use self::super::schema::*;

extern crate diesel;

#[derive(Serialize, Deserialize, Identifiable, Debug, Queryable)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    active: bool,
    sign_in_count: i64
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: i64
}

#[derive(Serialize, Deserialize, Debug, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: i64
}
