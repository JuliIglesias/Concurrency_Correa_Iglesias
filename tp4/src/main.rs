use std::io::Write;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use log_server::handle_connection;
use thread_pool_lib::ThreadPool;
use tokio::sync::Semaphore;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Crear un ThreadPool con un tamaño máximo de 4
    let pool = ThreadPool::new(4);

    // Datos compartidos para estadísticas
    let stats = Arc::new(Mutex::new(log_server::Stats::new()));

    // Crear un semáforo con un límite de 4 permisos
    let semaphore = Arc::new(Semaphore::new(4));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let stats = Arc::clone(&stats);
        let semaphore = Arc::clone(&semaphore);

        pool.execute(move || {
            let permit = semaphore.try_acquire();
            if permit.is_err() {
                // Si no hay permisos disponibles, devolver un error 429
                let response = "HTTP/1.1 429 Too Many Requests\r\n\r\nToo many files being processed";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
            handle_connection(stream, stats);
        })
    }
}
