extern crate page_api;

use diesel::pg::PgConnection;
use router::Router;
use std::sync::{Mutex, Arc};

pub fn initialize(router: &mut Router, database_connection: PgConnection) {
    let database_connection = Arc::new(Mutex::new(database_connection));

    page_api::mount_api(router, database_connection);
}
