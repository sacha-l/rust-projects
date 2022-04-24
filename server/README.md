# About this project

This was part of a "Rust Fundamentals" course where I learnt more about structs, enums, traits, networking, data structures and lifetimes. 

For this project, I built a simple HTTP server from scratch.

It implements the HTTP 1.1 protocol using the networking crate from the Rust standard library. 
It has the ability to safely handle requests and send responses, with appropriate error handling.

To run it:

1. Clone the the `server` folder of this repo
1. Run `cargo run`
1. Open up a web browser at http://127.0.0.1:8080

A couple cool things I learnt (you can read the code comments to learn more):
- Best practices for organizing code into modules (see how the `http` crate is organized).
- Stack and heap tracing for variables and string slices and why we pass string references.
- Lifetimes and writing functions that support them.
- What a directory traversal attack is.