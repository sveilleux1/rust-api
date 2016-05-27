extern crate webserver;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let hostname = env::var("HOSTNAME").expect("HOSTNAME must be set");
    let port = env::var("PORT").expect("PORT must be set");

    println!("Starting server at http://{}:{}...", hostname, port);
    webserver::start(&hostname, &port);
}
