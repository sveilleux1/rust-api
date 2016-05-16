extern crate router;
extern crate iron;
extern crate diesel;
extern crate page_service;

use diesel::pg::PgConnection;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::{Mutex, Arc};

mod controller;

pub fn mount_api(router: &mut Router, database_connection: Arc<Mutex<PgConnection>>) {
    let controller = controller::Controller::new(database_connection);
    let controller = Arc::new(Mutex::new(controller));

    let local_controller = controller.clone();
    router.get("/pages", move |request: &mut Request| -> IronResult<Response> {
        let controller = &mut local_controller.lock().unwrap();
        return controller.get_pages(request);
    });

    /*let local_controller = controller.clone();
    router.get("/pages3", move |request: &mut Request| -> IronResult<Response> {
        let controller = &mut local_controller.lock().unwrap();
        return controller.get_pages(request);
    });*/
}
