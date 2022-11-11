use chrono::{Duration, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use crate::schema::users;
use rocket::serde::{Deserialize,Serialize, json, json::Json};
use crate::domain::profile::Profile;

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


#[derive(Deserialize, AsChangeset, Default,Debug)]
#[table_name = "users"]
pub struct UpdateUserData {
    pub(crate) username: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) bio: Option<String>,
    pub(crate) image: Option<String>,

    // hack to skip the field
    #[column_name = "hash"]
    pub(crate)  password: Option<String>,
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
    pub fn to_profile(self, following: bool) -> Profile {
        Profile {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following,
        }
    }

}
