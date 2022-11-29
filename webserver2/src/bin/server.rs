use std::{
    fmt::{Debug, Display},
    io::{Write, Read},
    net::{TcpListener, TcpStream},
    thread,
    sync::{Arc, mpsc, Mutex},
};


fn main() -> std::io::Result<()> {
    let pool = ThreadPool::new(8).unwrap();
    let listener = TcpListener::bind("127.0.0.1:5001").unwrap();

    loop {
        let (tx, rx) = mpsc::channel();
        let (stream, _addr) = listener.accept().unwrap();
        let stream_clone = stream.try_clone().unwrap();

        pool.execute(move || {
            db_handle(stream_clone, tx);
        });

        pool.execute(move || {
            client_handle(stream, rx);
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}


fn client_handle(mut stream: TcpStream, rx: mpsc::Receiver<[u8; 64]>) {
    let buffer = rx.recv().unwrap();
    stream.write(&buffer).unwrap();
}

fn db_handle(mut stream: TcpStream, tx: mpsc::Sender<[u8; 64]>) {
    let mut buffer: [u8; 64] = [0; 64];
    stream.read(&mut buffer).unwrap();

    let mut db_stream = TcpStream::connect("127.0.0.1:5000").unwrap();
    db_stream.write(&buffer).unwrap();

    buffer = [0; 64];
    db_stream.read(&mut buffer).unwrap();

    tx.send(buffer).unwrap();
}

// fn operator(counter: Arc<RwLock<u8>>) {
//     thread::spawn(move || {
//         loop {
//             println!("Please press the number to check");
//             println!("1. current connected client number");
//             let mut buffer = String::new();
//             let _ = stdin().read_line(&mut buffer).unwrap();
//             if buffer.trim() == "1".to_owned() {
//                 println!("current connected client number : {}", *counter.read().unwrap());
//             }
//         }
//     });
// }

pub struct ThreadPool {
    sender: mpsc::Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<Self, ZeroWorkerError> {
        if size == 0 {
            return Err(ZeroWorkerError);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::new();

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(
            Self {
                sender,
                workers,
            }
        )
    }

    pub fn execute<F>(&self, func: F) 
        where   
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(func);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("[{} worker] : Shutting down", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("[{} worker] : Executing", id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("[{} worker] : To be terminated", id);
                        break;
                    },
                }
            }       
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub struct ZeroWorkerError;

impl Display for ZeroWorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Debug for ZeroWorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The number of worker should not be zero")
    }
}

impl std::error::Error for ZeroWorkerError {}
