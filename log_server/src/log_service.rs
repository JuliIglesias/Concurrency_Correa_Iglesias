use std::borrow::Cow;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::Stats;

pub fn upload(request: &Cow<str>, mut stream: &TcpStream, stats: Arc<Mutex<Stats>>) {
    let (file_name, file_content) = extract_file_content(request);

    save_stats_from_uploaded_documents(file_content, file_name.clone(), stats);

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Processed file: {}\n",
        file_name
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn save_stats_from_uploaded_documents(file_content: Vec<&str>, file_name: String, stats: Arc<Mutex<Stats>>){
    let exception_count = file_content
        .iter()
        .filter(|line| line.to_lowercase().contains("exception"))
        .count();

    {
        let mut stats = stats.lock().unwrap();
        stats.total_exceptions += exception_count;
        stats.files_processed += 1;
        stats.exceptions_per_file.push((file_name.clone(), exception_count));
    }
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

pub fn statistics(mut stream: &TcpStream, stats: Arc<Mutex<Stats>>) {

    let stats = stats.lock().unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\n{}",
        stats.format_stats()
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn not_found(mut stream: &TcpStream) {
    let response =
        "HTTP/1.1 400 Bad Request\r\n\
        Valid routes:\n\
        POST /upload - Upload a file for analysis\n\
        GET /stats - Show statistics";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}