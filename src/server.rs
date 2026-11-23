use std::{
    io::Write,
    net::{Shutdown, TcpListener, TcpStream},
};

use crate::{request::Request, route::Route};

pub struct Server {
    host: String,
    port: u16,
    handlers: Vec<(Route, Box<dyn Fn(Request)>)>,
}

impl Server {
    pub fn new(host: Option<&str>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or("localhost").to_owned(),
            port: port.unwrap_or(42069),
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
        let route = Route::new("/users/{id}");
        request.params = route.extract(&request.path);

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
