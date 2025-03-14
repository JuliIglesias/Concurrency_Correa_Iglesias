use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use http_server::leibniz_approximation;

use std::thread;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();

    if method =="GET" && path.starts_with("/pi/"){
        if let Some(n_str) = path.strip_prefix("/pi/"){
            if let Ok(n) = n_str.parse::<u64>(){
                let contents = fs::read_to_string("index.html").unwrap();
                // This is the line that calls the function for leibniz_approximation
                let (leibniz_result, leibniz_duration) = leibniz_approximation(n);
                let contents = contents
                    .replace("{leibniz_result}", &leibniz_result.to_string())
                    .replace("{N}", &n.to_string())
                    .replace("{time}", &leibniz_duration.to_string());

                handle_ok_response(stream, contents);
                return
            }
        }
    }

    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("404.html").unwrap();
    handle_404_response(&mut stream, status_line, contents);
}

fn handle_404_response(stream: &mut TcpStream, status_line: &str, contents: String) {
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents,
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_ok_response(mut stream: TcpStream, contents: String) {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
