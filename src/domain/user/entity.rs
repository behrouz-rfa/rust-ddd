use diesel::pg::Pg;
use diesel::prelude::*;
use crate::schema::users;
use rocket::serde::{Serialize,json,json::Json};

type Url = String;
#[derive(Queryable,Serialize, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<Url>,
    #[serde(skip_serializing)]
    pub hash: String,

}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}
