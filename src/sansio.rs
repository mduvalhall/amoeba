use std::collections::HashMap;

pub enum Progress {
    Incomplete,
    IncompleteBody { head: RequestHead, remaining: usize },
    Complete(Request),
}

pub fn build(packet: &Vec<u8>) -> Progress {
    match packet.windows(4).position(|w| w == b"\r\n\r\n") {
        Some(position) => {
            let head = RequestHead::from(&packet[..position]);
            if let Some(content_length) = head.headers.get("content-length") {
                let content_length = content_length
                    .parse::<usize>()
                    .expect("Content-Length should be an unsigned integer");
                if packet.len() >= content_length {
                    Progress::Complete(Request {
                        head,
                        body: Some(packet[position + 4..position + 4 + content_length].to_owned()),
                    })
                } else {
                    Progress::IncompleteBody {
                        head,
                        remaining: content_length - (packet.len() - (position + 4)),
                    }
                }
            } else {
                return Progress::Complete(Request { head, body: None });
            }
        }
        None => Progress::Incomplete,
    }
}

pub struct Request {
    head: RequestHead,
    body: Option<Vec<u8>>,
}

pub struct RequestHead {
    method: String,
    path: String,
    headers: HashMap<String, String>,
}

impl From<&[u8]> for RequestHead {
    fn from(packet: &[u8]) -> Self {
        let header = String::from_utf8_lossy(&packet);
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

        Self {
            method,
            path,
            headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sansio() {}
}
