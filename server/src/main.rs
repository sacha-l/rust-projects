#![allow(dead_code)]

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
mod website_handler;

mod server;
mod http;

fn main() {
    // assign a new instance of our server
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}