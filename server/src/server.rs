use std::net::TcpListener;
use std::io::Read;
// go to the root of a crate
use crate::http::Request;

// we have to state what in this mod is public
// what data needs to be associated 
pub struct Server {
    addr: String,
}

impl Server {
    // add a new associated function
    // Self and Server are aliases
    pub fn new(addr: String) -> Self {
        // pass the struct with its values
        Self { addr }
    }

    // run will take ownership of Server
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        
        // accept a connection
        // if accept fails we don't want the program to terminate
        loop {
            // much better than let (stream, addr) = res.unwrap();
            match listener.accept() {

                // get the tuple result and println
                Ok((mut stream, _)) => {
                    //stream.read(&mut )
                    // we need to store a large number of bytes
                    let mut buffer = [0; 1024];
                    // what if the data is larger ?
                    match stream.read(&mut buffer) {

                        Ok(_) => {
                            // log the request we receive
                            // convert the bytes to string
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }

                },
                //match the error case
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }

    }
}

// if you dont know the size of an array, pass in a reference to some 
// array and the compiler will know how to handle any number of elements
// e.g. a = [1, 2, 3];
// arr(&a[1..]);

// tests: echo "TEST" | netcat 127.0.0.1 8080