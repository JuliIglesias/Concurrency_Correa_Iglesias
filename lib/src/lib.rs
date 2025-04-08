use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::Instant;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();

    if method =="GET" && path.starts_with("/pi/"){
        if let Some(n_str) = path.strip_prefix("/pi/"){
            if let Ok(n) = n_str.parse::<u64>(){
                let contents = get_index_html().unwrap();
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
    let contents = get_404_html().unwrap();
    handle_404_response(&mut stream, status_line, contents);
}

pub fn get_index_html() -> Result<String, std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets/index.html");
    fs::read_to_string(path)
}

pub fn get_404_html() -> Result<String, std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets/404.html");
    fs::read_to_string(path)
}

    pub fn handle_404_response(stream: &mut TcpStream, status_line: &str, contents: String) {
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents,
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn handle_ok_response(mut stream: TcpStream, contents: String) {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


pub fn leibniz_approximation(n: u64) -> (f64, f64) {
    let now = Instant::now();
    let mut result: f64 = 0.0;

    for i in 0..=n {
        result += leibniz_term(i);
    }

    let total_duration: f64 = now.elapsed().as_secs_f64();

    (result, total_duration)
}

fn leibniz_term(n: u64) -> f64 {
    let numerator: f64 = 4.0 * if n % 2 == 0 { 1.0 } else { -1.0 };
    let dividend: f64 = 2.0 * n as f64 + 1.0;

    numerator / dividend
}
