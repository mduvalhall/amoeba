#![allow(dead_code, unused_variables)]

struct Request {
    head: Head,
    body: Option<Body>,
}

struct Head {
    data: Vec<u8>,
}

struct Body {
    data: Vec<u8>,
}
