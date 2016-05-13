extern crate webserver;

fn main() {
    let hostname = "localhost".to_string();
    let port = 3002;

    println!("Starting server at http://{}:{}...", hostname, port);
    webserver::start(&hostname, &port);
}
