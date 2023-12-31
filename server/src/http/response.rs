use super::StatusCode;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            None => "",
            Some(msg) => msg,
        };

        write!(stream, "HTTP/1.1 {} \r\n\r\n{}", self.status_code, body)
    }
}
