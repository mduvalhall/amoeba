use std::{
    io::Write,
    net::{Shutdown, TcpListener, TcpStream},
};

use crate::request::Request;

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new(host: Option<&str>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or("localhost").to_owned(),
            port: port.unwrap_or(42069),
        }
    }

    pub fn run(self) -> () {
        let address = format!("{}:{}", self.host, self.port);
        println!("Amoeba server running on {}", address);
        let listener = TcpListener::bind(&address).unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            self.handle_request(&mut stream);
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }

    fn handle_request(&self, stream: &mut TcpStream) -> () {
        let request = Request::from(&mut *stream);
        println!("{:#?}", request);
        self.handle_response(stream);
    }

    fn handle_response(&self, stream: &mut TcpStream) -> () {
        stream.write(self.ok()).unwrap();
    }

    fn ok(&self) -> &'static [u8] {
        b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nhello world"
    }
}
