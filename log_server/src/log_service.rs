use crate::stats_struct::Stats;

use std::borrow::Cow;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{OnceLock, RwLock};

static GLOBAL_STATS: OnceLock<RwLock<Stats>> = OnceLock::new();

fn get_global_stats() -> &'static RwLock<Stats> {
    GLOBAL_STATS.get_or_init(|| RwLock::new(Stats::new()))
}

pub fn upload(request: &Cow<str>, mut stream: &TcpStream) {
    let (file_name, file_content) = extract_file_content(request);

    save_stats_from_uploaded_documents(file_content, file_name.clone());

    let body = format!("HTTP/1.1 200 OK\r\n\
    Processed file: {}", file_name
    );

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
}

fn save_stats_from_uploaded_documents(file_content: Vec<&str>, file_name: String){
    let exception_count = file_content
        .iter()
        .filter(|line| line.to_lowercase().contains("exception"))
        .count();

    let mut stats = get_global_stats().write().unwrap();
    stats.total_exceptions += exception_count;
    stats.files_processed += 1;
    stats.exceptions_per_file.push((file_name.clone(), exception_count));
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

    //here fix this (juli)
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

pub fn statistics(mut stream: &TcpStream) {

    let stats = get_global_stats().read().unwrap();

    let body = format!("\
        HTTP/1.1 200 OK\r\n\
        {}",
        stats.format_stats()
    );

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        {}",
        body
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn not_found(mut stream: &TcpStream) {
    let body =
        "HTTP/1.1 400 Bad Request\r\n\
        Valid routes:\r\n\
        POST /upload - Upload a file for analysis\r\n\
        GET /stats - Show statistics\r\n";

    let response = format!(
        "HTTP/1.1 400 Bad Request\r\n\
        {}",
        body
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
