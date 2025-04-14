mod routes_controller;
mod log_service;

use std::io::Read;
use std::net::TcpStream;
use routes_controller::handle_request;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    handle_request(&mut buffer, &stream);

    // let request = String::from_utf8_lossy(&buffer);
    // println!("{}", request);
}
