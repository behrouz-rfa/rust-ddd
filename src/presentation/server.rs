//! The Server type for laucnh a hnadler
//! and Server is going to have some configuration for run the application
//! such as ['port','address','db config]
//! Examples
//!````
//!  Server::lucnch().await?;
//!````

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
use crate::application::services::domain::article::service::ArticleService;
use crate::application::services::domain::profile::service::ProfileService;
use crate::infrastructure::domain::user::repository::UserRepository;
use crate::infrastructure::domain::article::repository::ArticleRepository;
use crate::infrastructure::domain::profile::repository::ProfileRepository;
use crate::presentation::http_handler::users::{insert_users, login_user, update_user};
use crate::presentation::http_handler::article::{create_article, get_article};
use crate::presentation::config::{AppState, from_env};


#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

/// Represent to create new server with this struct
pub struct Server {
    /// load config from env or toml file
    _config: String,
}


/// The declartion of the `ServiceState` use for add
/// all Dependency like Service and repository
/// # Examples
/// ```
///   let user_repo = UserRepository::new(pool.clone());
///   let user_service = UserService::new(user_repo);
///   ServiceState { service: Atomic::new(user_service) }
/// ```
pub struct ServiceState<R> {
    pub service: Atomic<R>,
}

impl Server {
    /// We create a custom builder for launch rocket as http http_handler
    /// Some Configuration is come from `AppState` for mor details go to  the `config` file
    /// In the launch we need to pass all dependancy as a State
    /// # Exmples
    /// ````
    ///         let pool = get_dbpool();
    ///         let user_repo = UserRepository::new(pool.clone());
    ///         let user_service = UserService::new(user_repo);
    ///         ServiceState { service: Atomic::new(user_service) }
    ///
    /// ```
    pub async fn launch() -> Result<(), rocket::Error> {

        //get db pool for `db`
        //you can clone th DbPool and pass them to  multi Repository
        let pool = get_dbpool();
        //create a new instance of UserRepository
        let user_repo = UserRepository::new(pool.clone());
        // Create UserService and pass user repository as an Dependency for UserService
        let user_service = UserService::new(user_repo);

        //create a new instance of ArticleRepository
        let article_repo = ArticleRepository::new(pool.clone());
        // Create UserService and pass user repository as an Dependency for ArticleService
        let articel_service = ArticleService::new(article_repo);


        //create a new instance of ProfileRepository
        let profile_repo = ProfileRepository::new(pool.clone());
        // Create UserService and pass user repository as an Dependency for ProfileService
        let profile_service = ProfileService::new(profile_repo);

        //Build rocket launcher adn pass all route and state to the manage and attach
        let _rocket = rocket::custom(from_env())
            .manage(ServiceState { service: Atomic::new(user_service) })
            .manage(ServiceState { service: Atomic::new(articel_service) })
            .manage(ServiceState { service: Atomic::new(profile_service) })
            .attach(AppState::manage())
            .mount("/api", routes![
                insert_users,
                login_user,
                update_user,
                create_article,get_article
            ])

            .register("/", catchers![not_found])
            .launch()
            .await?;
        Ok(())
    }
}

