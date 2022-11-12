//! rust-ddd   is used to present how I find implementing DDD in Rust projects works out
//! This codebase was created to demonstrate a fully Rust Domain-Driven-Design  built with Rocket including CRUD operations, authentication, routing, pagination, and more.
//!  We've gone to great lengths to adhere to the Rocket community styleguides & best practices.
//!  For more information on how to this works with other frontends/backends, head over to the RealWorld repo.
//! more feature is going to add on this project
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(let_else)]


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

   Server::launch().await?;
   Ok(())

}

