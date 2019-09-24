use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock()
                    .unwrap()
                    .recv()
                    .unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} executing a job", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} terminating", id);
                        break;
                    }
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for k in 0..size {
            let w = Worker::new(k, Arc::clone(&receiver));
            workers.push(w);
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down all workers");
        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shut down all workers");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}