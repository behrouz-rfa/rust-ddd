use std::sync::Arc;
use std::sync::atomic::Ordering;
use crossbeam::epoch::Atomic;
use rocket::serde::json::{json, Value};

use dotenv::dotenv;
use rocket::{Request, State};
use rocket::fairing::AdHoc;
use rocket::request::{FromRequest, Outcome};

use crate::infrastructure::db::{DbPool, get_dbpool};
use crate::application::services::domain::user::service::UserService;
use crate::infrastructure::domain::user::repository::UserRepository;
use crate::presentation::handler::users::{insert_users, login_user};
use crate::presentation::config::{AppState, from_env};


#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub struct Server {
    _port: u16,
}

pub struct ServiceState<R> {
  pub  service: Atomic<R>,
}

// #[rocket::async_trait]
// impl<'r, R> FromRequest<'r> for ServiceState<R> {
//     type Error = ();
//
//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         let t = request.guard::<ServiceState<R>>().await;
//         t
//     }
// }


impl Server {
    pub async fn lucnch() -> Result<(), rocket::Error> {
        let pool = get_dbpool();
        let user_repo = UserRepository::new(pool.clone());
        let user_service = UserService::new(user_repo);

        let _rocket = rocket::custom(from_env())
            .manage(ServiceState { service: Atomic::new(user_service) })
            .attach(AppState::manage())
            .mount("/api", routes![insert_users,login_user])


            .register("/", catchers![not_found])
            .launch()
            .await?;
        Ok(())
    }
}

