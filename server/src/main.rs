#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod website_handler;
mod server;
mod http;

fn main() {
    // read an environment variable from the compiler using env! macro
    // here, we're creating a default path and appending 'public' to it
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // you could use the std::env to set a single environment variable 
    // here we use unwrap_or to use the default path if it can't unwrap
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path is: {}", public_path);

    // assign a new instance of our server
    let server = Server::new("127.0.0.1:8080".to_string());
    // now WebsiteHandler has a fn to take some String for path
    server.run(WebsiteHandler::new(public_path));
}