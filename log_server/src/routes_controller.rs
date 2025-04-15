use crate::{log_service, Stats};
use std::borrow::Cow;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use log_service::upload;
use log_service::stats;

pub fn handle_request(buffer: &mut [u8; 1024], mut stream: &TcpStream, stats: Arc<Mutex<Stats>>) {
    let request: Cow<str> = String::from_utf8_lossy(&buffer[..]);

    let (method, path) = parse_request(&request);

    handle_route(method, path, &request, stream, stats);
}

fn parse_request(request: &Cow<str>) -> (String, String) {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap().to_string();
    let path = parts.next().unwrap().to_string();

    (method, path)
}

fn handle_route(method: String,
                path: String,
                request: &Cow<str>,
                mut stream: &TcpStream,
                stats: Arc<Mutex<Stats>>
) {
    if method == "POST" && path.starts_with("/upload"){
        upload(request, stream, stats);
    } else if method == "GET" && path.starts_with("/stats"){
        stats(stream, stats);
    } else{
        not_found(stream);
    }
}
