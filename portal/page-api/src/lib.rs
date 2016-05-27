extern crate router;
extern crate iron;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate page_service;

use diesel::pg::PgConnection;
use iron::prelude::*;
use router::Router;
use std::sync::{Mutex, Arc};
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

mod controller;

pub fn mount_api(router: &mut Router, database_connection_pool: &Pool<ConnectionManager<PgConnection>>) {
    let controller = controller::Controller::new(database_connection_pool.clone());
    let controller = Arc::new(Mutex::new(controller));

    let local_controller = controller.clone();
    router.get("/pages", move |request: &mut Request| -> IronResult<Response> {
        let mut controller = local_controller.lock().unwrap();
        return controller.get_pages(request);
    });

    let local_controller = controller.clone();
    router.get("/pages3", move |request: &mut Request| -> IronResult<Response> {
        let mut controller = local_controller.lock().unwrap();
        return controller.get_pages(request);
    });
}
