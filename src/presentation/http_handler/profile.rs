use std::sync::Arc;
use std::sync::atomic::Ordering;
use diesel::serialize::IsNull::No;
use rocket::request::FromRequest;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use crate::infrastructure::domain::profile::repository::ProfileRepository;
use crate::application::services::domain::profile::{service::ProfileService};
use rocket::State;
use crate::application::services::domain::article::dto::CreateArticleDto;

use crate::application::services::domain::user::dto::{NewUserDto, UpdateUserDto};
use crate::domain::user::entity::UpdateUserData;
use crate::error::DbError;
use crate::errors::{Errors, FieldValidator};
use crate::presentation::config::AppState;
use crate::presentation::middleware::auth::Auth;
use crate::presentation::server::ServiceState;


#[get("/profiles/<username>")]
pub fn get_profile(username: String,
                   auth: Auth,
                   user_service: &State<ServiceState<ProfileService<ProfileRepository>>>,
) -> Result<Value, Errors> {
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("error", "server error")]));;
        }
        let t = unsafe { shared.as_ref() };

        return t.unwrap().find(&username, auth.id).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "a problem occur while getting user profile")])
            }
            );
    }


    Err(Errors::new(&[("err", "a problem occur while getting user profile")]))
}

#[post("/profiles/<username>/follow")]
pub fn follow(username: String,
              auth: Auth,
              user_service: &State<ServiceState<ProfileService<ProfileRepository>>>,
) -> Result<Value, Errors> {
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("error", "server error")]));
        }
        let t = unsafe { shared.as_ref() };

        return t.unwrap().follow(&username, auth.id).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "a problem occur while follow user")])
            }
            );
    }


    Err(Errors::new(&[("err", "a problem occur while follow user")]))
}

#[delete("/profiles/<username>/follow")]
pub fn unfollow(username: String,
                auth: Auth,
                user_service: &State<ServiceState<ProfileService<ProfileRepository>>>,
) -> Result<Value, Errors> {
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("error", "server error")]));
        }
        let t = unsafe { shared.as_ref() };

        return t.unwrap().unfollow(&username, auth.id).map(|article| json!(article))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "a problem occur while unfollow user")])
            }
            );
    }


    Err(Errors::new(&[("err", "a problem occur while unfollow user")]))
}
