use std::borrow::Cow;
use std::net::TcpStream;

pub fn handle_request(buffer: &mut [u8; 1024], mut stream: &TcpStream) {
    let request = String::from_utf8_lossy(&buffer[..]);

    let (method, path) = parse_request(&request);

    handle_route(method, path, buffer, stream);
}

fn parse_request(request: &Cow<str>) -> (String, String) {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap().to_string();
    let path = parts.next().unwrap().to_string();

    (method, path)
}

fn handle_route(method: String, path: String, buffer: &mut [u8; 1024], mut stream: &TcpStream) {
    println!("Hello, we are handle route");

    if method == "POST" && path.starts_with("/upload"){
        println!("omg we are in {}", path);

        upload(buffer, stream)
    }

    if method == "GET" && path.starts_with("/stats"){
        stats(stream)
    }
}

fn upload(buffer: &mut [u8; 1024], mut stream: &TcpStream) {
    extract_file_content(buffer);
}


pub fn extract_file_content(buffer: &mut [u8; 1024]) {
    let request = String::from_utf8_lossy(buffer);
    let headers_end = request.find("\r\n\r\n").unwrap() + 4;
    let headers = &request[..headers_end];
    let body = &buffer[headers_end..];


    if let Some(content_type) = headers.lines().find(|line| line.starts_with("Content-Type:")) {
        if let Some(boundary) = content_type.split("boundary=").nth(1) {
            let boundary = format!("--{}", boundary.trim());
            for part in body.split(|b| b == &b'\r' || b == &b'\n') {
                println!("in for:{}", String::from_utf8_lossy(part));
                if part.starts_with(boundary.as_bytes()) {

                    if let Ok(part_str) = std::str::from_utf8(part) {
                        for line in part_str.lines() {
                            if line.starts_with("Content-Disposition:") {
                                println!("Content-Disposition: {}", line);
                                if line.contains("filename=") {
                                    let file_start = part
                                        .windows(4)
                                        .position(|window| window == b"\r\n\r\n")
                                        .map(|pos| pos + 4)
                                        .unwrap_or(0);
                                    let file_content = &part[file_start..];
                                    if let Ok(content) = std::str::from_utf8(file_content) {
                                        println!("File content: {}", content);
                                    }
                                }
                                break; // Exit the loop once the header is found
                            }
                        }
                    } else {
                        println!("Failed to convert part to UTF-8");
                    }
                    println!("sera q no toma y da error??");

                }
            }
        }
    }
}

fn stats(mut stream: &TcpStream) {

}
