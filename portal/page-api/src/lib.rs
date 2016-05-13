extern crate router;
extern crate iron;
extern crate page_service;

use iron::prelude::*;
use router::Router;

mod controller;

pub fn mount_api(router : &mut Router) {
    router.get("/pages/", |request: &mut Request| -> IronResult<Response> {
        let controller = controller::Controller::new();
        return controller.get_pages(request);
    });
}
