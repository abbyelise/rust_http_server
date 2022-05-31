use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::convert::TryFrom;
// crate means the root of the directory/crate/project
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    // not including '&self' means it's similar to a static function
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        // unwrap failure should fail the program
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));

                            match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    
                                },
                                Err(e) => println!("Failed to convert request: {}", e),
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
