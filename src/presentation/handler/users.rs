use std::sync::Arc;
use std::sync::atomic::Ordering;
use rocket::request::FromRequest;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use crate::infrastructure::domain::user::repository::UserRepository;
use crate::application::services::domain::user::{service::UserService};
use rocket::State;
use crate::application::services::domain::user::dto::NewUserDto;
use crate::error::DbError;
use crate::errors::{Errors, FieldValidator};
use crate::presentation::config::AppState;
use crate::presentation::server::ServiceState;


#[derive(Debug, Deserialize, PartialEq, Eq, Validate)]
#[serde(crate = "rocket::serde")]
pub struct NewUserData {
    #[validate(length(min = 1))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 1))]
    pub password: Option<String>,
}


#[post("/users/insert", format = "json", data = "<new_user>")]
pub fn insert_users(new_user: Json<NewUserData>, state: &State<AppState>, user_service: &State<ServiceState<UserService<UserRepository>>>) -> Result<Value, Errors> {
    let user = new_user.into_inner();

    let mut extractor = FieldValidator::default();
    let email = extractor.extract("email", user.email);
    let username = extractor.extract("username", user.username);
    let password = extractor.extract("password", user.password);
    extractor.check()?;

    // let secret = state.secret.clone();
    let g = &crossbeam::epoch::pin();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        let secret = state.secret.clone();
        return t.unwrap().create(NewUserDto {
            email,
            username,
            password,
        }).map(|user| json!(user.to_jwt_user(&secret)))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user already exist")])
            }
            );
    }


    Err(Errors::new(&[("email or password", "is invalid")]))
}


#[derive(Debug, Deserialize, PartialEq, Eq, Validate)]
pub struct LoginUserData {
    pub email: Option<String>,
    pub password: Option<String>,
}


#[post("/users/login", format = "json", data = "<user>")]
pub fn login_user(
    user: Json<LoginUserData>,
    state: &State<AppState>,
    user_service: &State<ServiceState<UserService<UserRepository>>>,
) -> Result<Value, Errors> {
    let user = user.into_inner();

    let mut extractor = FieldValidator::default();
    let email = extractor.extract("email", user.email);
    let password = extractor.extract("password", user.password);
    extractor.check()?;


    // let secret = state.secret.clone();
    let g = &crossbeam::epoch::pin();
    let secret = state.secret.clone();
    if let shared = user_service.service.load(Ordering::Relaxed, g) {
        if shared.is_null() {
            return Err(Errors::new(&[("email or password", "is invalid")]));
        }
        let t = unsafe { shared.as_ref() };
        return t.unwrap().find_by(NewUserDto {
            email,
            username: "".to_string(),
            password,
        }).map(|user| json!(user.to_jwt_user(&secret)))
            .map_err(|err| {
                println!("{}", err);

                Errors::new(&[("err", "user not exist")])
            }
            );
    }


    Err(Errors::new(&[("email or password", "is invalid")]))
}



