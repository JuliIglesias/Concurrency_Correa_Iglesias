use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use log_server::handle_connection;
use thread_pool_lib::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Crear un ThreadPool con un tamaño máximo de 4
    let pool = ThreadPool::new(4);

    // Datos compartidos para estadísticas
    let stats = Arc::new(Mutex::new(log_server::Stats::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let stats = Arc::clone(&stats);

        pool.execute(move || {
            handle_connection(stream, stats);
        })
    }
}
