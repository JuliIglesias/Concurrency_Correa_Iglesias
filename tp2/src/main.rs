use std::net::TcpListener;
use std::thread;
use lib::handle_connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
