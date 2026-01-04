use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// Single Worker struct
/// Contains an id (usize) and a thread of type Option<thread::JoinHandle<()>>
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}


impl Worker {
    /// Worker::new()
    /// Takes an 
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(
            move || loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        );
        Worker { id: id, thread: Some(thread) }
    }
}

/// Job type alias of Box<dyn FnOnce + Send + 'static>
type Job = Box<dyn FnOnce() + Send + 'static>;

/// ThreadPool contains a vector of Workers
/// and a sender of type Option<mpsc::Sender<Job>>
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers: workers, sender: Some(sender)}
    }

    pub fn execute<F> (&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}