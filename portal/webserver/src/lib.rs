#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate router;
extern crate r2d2;
extern crate r2d2_diesel;

use iron::prelude::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2::*;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

mod portal_router;

pub fn start(hostname: &String, port: &String) {
    let mut router = router::Router::new();
    let database_connection_pool = create_database_conection_pool();

    portal_router::initialize(&mut router, &database_connection_pool);

    let server_url = format!("{}:{}", hostname, port).to_string();
    Iron::new(router).http(&*server_url).unwrap();
}

fn create_database_conection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    return r2d2::Pool::new(config, manager).expect("Failed to create pool.");
}
