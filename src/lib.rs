extern crate diesel;
#[macro_use]
extern crate rocket;

mod models;
mod repositories;
mod schema;

pub mod auth;
pub mod commands;
pub mod routes;
