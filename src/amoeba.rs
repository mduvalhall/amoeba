use std::{
    io::Write,
    net::{Shutdown, TcpListener, TcpStream},
};

use crate::{request::Request, route::Route};

pub struct Amoeba {
    host: String,
    port: u16,
    handlers: Vec<(Route, Box<dyn Fn(Request)>)>,
}

impl Amoeba {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_owned(),
            port,
            handlers: Vec::new(),
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

    pub fn add_handler(&mut self, route: Route, handler: fn(Request)) {
        self.handlers.push((route, Box::new(handler)))
    }

    fn handle_request(&self, stream: &mut TcpStream) -> () {
        let mut request = Request::from(&mut *stream);
        for (route, handler) in self.handlers.iter() {
            let params = route.extract(&request.path);
            if params.is_some() {
                request.params = params;
                handler(request);
                break;
            }
        }
        self.handle_response(stream);
    }

    fn handle_response(&self, stream: &mut TcpStream) -> () {
        stream.write(self.ok()).unwrap();
    }

    fn ok(&self) -> &'static [u8] {
        b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nhello world"
    }
}

impl Default for Amoeba {
    fn default() -> Self {
        Self::new("localhost", 42069)
    }
}
