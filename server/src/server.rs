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
    }
}