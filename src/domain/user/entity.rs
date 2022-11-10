use chrono::{Duration, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use crate::schema::users;
use rocket::serde::{Serialize, json, json::Json};

use crate::presentation::middleware::auth::Auth;



#[derive(Queryable, Identifiable, Serialize, PartialEq, Eq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    #[serde(skip_serializing)]
    pub hash: String,

}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    username: &'a str,
    email: &'a str,
    bio: Option<&'a str>,
    image: Option<&'a str>,
    token: String,
}


#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

impl User {
    pub fn to_jwt_user(&self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(60);
        let token = Auth {
            id: self.id,
            username: self.username.clone(),
            exp: exp.timestamp(),
        }.token(secret);

        UserAuth {
            username: &self.username,
            email: &self.email,
            bio: self.bio.as_ref().map(String::as_str),
            image: if let Some(v) = &self.image {
                Some(v.as_str())
            } else { None },
            token,
        }
    }
}
