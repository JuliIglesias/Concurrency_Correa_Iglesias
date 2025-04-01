use std::net::TcpListener;
use std::thread;
use lib::handle_connection;
use tp3::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7880").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}