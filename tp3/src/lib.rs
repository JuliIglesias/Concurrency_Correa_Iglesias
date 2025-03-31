use std::sync::{mpsc, Arc, Mutex};
use std::thread;


// Estructura que va a contener el id del worker y el thread que va a ejecutar el trabajo
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// Va a administrar el conjunto de threads, y va a ser el encargado de enviar los trabajos a los threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {

    // Crea un nuevo ThreadPool con un tamaño dado, e inicializa el canal que va a ser utilizado para enviar a los threads
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // Toma una función que va a ser ejecutada por un thread del pool y envía el trabajo a través del canal
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


// Se usa cuando el ThreadPool sale del scope, para asegurarse de que todos los threads se cierren correctamente destruyéndose
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Este al crearse ejecuta un loop que espera a recibir un trabajo y lo ejecuta cuando lo recibe
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}