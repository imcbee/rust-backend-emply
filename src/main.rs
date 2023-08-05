#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod employees;
mod error_handler;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Helps to manage our environment variables
    db::init(); // A function that initiates the database connection, which is referenced from another file called db.rs
    let mut listenfd = ListenFd::from_env(); // Restarts the server when changes is detected in files
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes)); // A function that abstracts all routes used in this project
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
