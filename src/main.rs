#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};
use crate::presentation::server::Server;


#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

mod infrastructure;
mod presentation;
mod domain;
mod schema;
mod application;
mod error;
mod errors;

#[rocket::main]
async fn main()-> Result<(), rocket::Error>{
   Server::lucnch().await?;
   Ok(())

}

