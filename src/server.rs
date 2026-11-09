use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

pub struct Server {
    address: String,
    handlers: HashMap<String, Box<dyn Fn() -> ()>>,
}

impl Server {
    pub fn new(host: Option<&str>, port: Option<u16>) -> Self {
        let host = host.unwrap_or("localhost").to_owned();
        let port = port.unwrap_or(42069);
        Self {
            address: format!("{host}:{port}"),
            handlers: HashMap::new(),
        }
    }

    pub fn run(self) -> () {
        println!("Amoeba server running on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            self.handle_request(&mut stream);
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }

    fn handle_request(&self, stream: &mut TcpStream) -> () {
        let mut buf = [0u8; 1024];
        let bytes = stream.read(&mut buf).unwrap();
        let request = String::from_utf8_lossy(&buf[..bytes]);
        println!("{request}");
        self.handle_response(stream);
    }

    fn handle_response(&self, stream: &mut TcpStream) -> () {
        stream.write(self.ok()).unwrap();
    }

    fn ok(&self) -> &'static [u8] {
        b"HTTP/1.1 200 OK\r\n\r\n"
    }
}
