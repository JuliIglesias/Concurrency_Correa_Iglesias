use crate::{log_service, stats_struct};
use std::borrow::Cow;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use log_service::upload;
use log_service::statistics;
use log_service::not_found;

pub fn handle_request(buffer: &mut [u8; 1024], mut stream: &TcpStream) {
    let request: Cow<str> = String::from_utf8_lossy(&buffer[..]);

    let (method, path) = parse_request(&request);

    handle_route(method, path, &request, stream);
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
                mut stream: &TcpStream
) {
    if method == "POST" && path.eq("/upload"){
        upload(request, stream);
    } else if method == "GET" && path.eq("/stats"){
        statistics(stream);
    } else{
        not_found(stream);
    }
}
