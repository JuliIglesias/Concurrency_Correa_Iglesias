use std::net::TcpListener;
use lib::handle_connection;
use thread_pool_lib::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7880").unwrap();

    let pool = ThreadPool::new(24);

    for stream in listener.incoming() {
        println!("\nNew connection");
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
