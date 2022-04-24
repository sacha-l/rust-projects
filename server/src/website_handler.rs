use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;
use std::fs;

// allow handler to take a path to read HTML files
pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        // we first concatonate the path
        let path = format!("{}/{}", self.public_path, file_path);
        // we then get the canonical path with .. removed
        // we make sure the path starts with the root directory
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    // ok to safely read the path
                    // this returns a string so we need to convert it to an Option using .ok()
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        //Response::new(StatusCode::Ok, Some("<h1>Test</>".to_string()))
        // handle all possible requests
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                // we want to match any path and render the page if it exists, throwing an error if not
                // below works, but introduces a directory traversal attack: we don't validate the user input at all
                // we hope the user doesn't do: GET /../../system_files (they have access to the entire pc running the server)
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                }
                _ => Response::new(StatusCode::NotFound, None),
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}