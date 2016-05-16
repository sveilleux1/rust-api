#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate router;

use iron::prelude::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod portal_router;

pub fn start(hostname: &String, port: &i32) {
    let mut router = router::Router::new();
    let mut database_connection = establish_connection();

    portal_router::initialize(&mut router, database_connection);

    let server_url = format!("{}:{}", hostname, port).to_string();
    Iron::new(router).http(&*server_url).unwrap();
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
