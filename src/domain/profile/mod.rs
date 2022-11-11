pub mod entity;
pub(crate) mod repository;
use rocket::serde::Serialize;
#[derive(Serialize)]
pub struct Profile {
    pub  username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

