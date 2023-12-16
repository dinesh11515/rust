use crate::http::Request;
use crate::http::{ParseError, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, req: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(
            StatusCode::BadRequest,
            Some("<h1>It's a Bad Request</h1>".to_string()),
        )
    }
}
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening to {}", self.address);

        let listener = TcpListener::bind(self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Response Text : {}", String::from_utf8_lossy(&buffer));
                            let respone = match Request::try_from(&buffer[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = respone.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Failed fetching from stream {}", e);
                        }
                    };
                }
                Err(e) => println!("Error Accepting the listener {}", e),
            }
        }
    }
}
