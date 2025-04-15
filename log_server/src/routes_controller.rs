use crate::{log_service};
use std::borrow::Cow;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{OnceLock};
use log_service::upload;
use log_service::statistics;
use log_service::not_found;
use tokio::sync::Semaphore;

static UPLOAD_SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();

fn get_upload_semaphore() -> &'static Semaphore {
    UPLOAD_SEMAPHORE.get_or_init(|| Semaphore::new(4))
}

pub fn handle_request(buffer: &mut [u8; 1024], stream: &TcpStream) {
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
        let semaphore = get_upload_semaphore();
        let permit = semaphore.try_acquire();

        if permit.is_err() {

            let body =
                "HTTP/1.1 429 Too Many Requests\r\n\
                Too many files being processed";

            let response = format!(
                "HTTP/1.1 200 OK\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {}\r\n\
                \r\n\
                {}",
                body.len(),
                body
            );

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
        upload(request, stream);
    } else if method == "GET" && path.eq("/stats"){
        statistics(stream);
    } else{
        not_found(stream);
    }
}
