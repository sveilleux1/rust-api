extern crate page_api;

use diesel::pg::PgConnection;
use router::Router;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

pub fn initialize(router: &mut Router, database_connection_pool: &Pool<ConnectionManager<PgConnection>>) {
    page_api::mount_api(router, database_connection_pool);
}
