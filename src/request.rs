use std::{collections::HashMap, fmt::Debug, io::Read, net::TcpStream, str::FromStr};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub params: Option<HashMap<String, String>>,
}

impl Request {
    pub fn param<T>(&self, param: &str) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        match &self.params {
            Some(params) => params.get(param).unwrap().parse().unwrap(),
            None => panic!(),
        }
    }
}

impl<'a> From<&mut TcpStream> for Request {
    fn from(stream: &mut TcpStream) -> Self {
        let mut packet = Vec::<u8>::new();
        let header_len = loop {
            let mut buf = [0; 8196];
            let n = stream.read(&mut buf).expect("Failed to read from stream");
            packet.extend(&buf[..n]);
            if let Some(position) = packet.windows(4).position(|w| w == b"\r\n\r\n") {
                break position;
            }
            if n == 0 {
                panic!("Stream closed too early")
            }
        };

        let header = String::from_utf8_lossy(&packet[..header_len]);
        let mut lines = header.lines();

        let line = lines.next().expect("Request line missing");
        let mut parts = line.split(" ");
        let method = parts.next().expect("Method missing").to_owned();
        let path = parts.next().expect("Path missing").to_owned();
        let _version = parts.next().expect("Version missing");

        let mut headers = HashMap::new();
        for line in lines {
            let (name, value) = line.split_once(":").expect("Invalid header format");
            headers.insert(
                name.trim().to_lowercase().to_owned(),
                value.trim().to_lowercase().to_owned(),
            );
        }

        let mut body = None;
        if let Some(body_len) = headers.get("content-length") {
            let body_len = body_len
                .parse::<usize>()
                .expect("content-length must be usize");
            while (packet.len() - header_len - 4) < body_len {
                let mut buf = [0; 8196];
                let n = stream.read(&mut buf).expect("Failed to read from stream");
                if n == 0 {
                    panic!("Stream closed too early");
                }
                packet.extend(&buf[..n]);
            }
            body = Some(packet[header_len + 4..].to_vec());
        }

        Self {
            method,
            path,
            headers,
            body,
            params: None,
        }
    }
}
