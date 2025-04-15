use std::net::TcpListener;
use log_server::handle_connection;
use thread_pool_lib::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Crear un ThreadPool con un tamaño máximo de 4
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        })
    }
}
