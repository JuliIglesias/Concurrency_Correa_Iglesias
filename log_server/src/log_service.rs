use std::borrow::Cow;
use std::net::TcpStream;

pub fn upload(request: &Cow<str>, mut stream: &TcpStream) {
    let (file_name, file_content) = extract_file_content(request);

}

fn extract_file_content<'a>(request: &'a Cow<str>) -> (String, Vec<&'a str>) {
    let headers_end = request.find("\r\n\r\n").unwrap() + 4;
    let headers = &request[..headers_end];
    let body = &request[headers_end..];

    println!("full request: {}", request);
    println!("headers: {}", headers);
    println!("body: {}", body);

    let mut body_lines = body.lines();
    body_lines.next(); // Discard start boundary.
    let file_name = body_lines.next().unwrap().split("filename=").nth(1).unwrap().trim().trim_matches('"');
    body_lines.next(); // Discard Content-Type.
    body_lines.next(); // Discard blank.
    let mut content_lines: Vec<&str> = body_lines.collect();
    content_lines.pop();
    content_lines.pop(); // Remove the end boundary.

    println!("file_name: {}", file_name);
    println!("content_lines: {:?}", content_lines);

    (file_name.to_string(), content_lines)
}

pub fn stats(mut stream: &TcpStream) {

}