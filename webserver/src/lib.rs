extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;

mod portal_router;

pub fn start(hostname: &String, port: &i32) {
    let mut router = router::Router::new();
    //portal_router::initialize(router);
    
    let server_url = format!("{}:{}", hostname, port).to_string();
    Iron::new(router).http(&*server_url).unwrap();
}
