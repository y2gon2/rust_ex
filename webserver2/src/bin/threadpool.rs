// 이건 공식문서 threadpool 만들기와 동일내용

// use std::{
//     fmt::{Display, Debug},
//     fs::File,
//     io::{Read, Write},
//     net::{TcpListener, TcpStream},  
//     sync::{Arc, mpsc, Mutex},
//     thread,
//     time::Duration,
// };

fn main() {}

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

//     let pool = ThreadPool::new(5).unwrap();

//     for stream in listener.incoming().take(4) {
//         let stream = stream.unwrap();

//         pool.execute(|| {
//             connection_handler(stream);
//         });
//     }
// }

// fn connection_handler(mut stream: TcpStream) {
//     let mut buffer: [u8; 512] = [0; 512];
//     stream.read(&mut buffer).unwrap();
    
//     let get = b"GET HTTP/1.1\r\n";
//     let sleep = b"GET/sleep HTTP/1.1\r\n";

//     let (status_line, filename) = if buffer.starts_with(get) {
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else if buffer.starts_with(sleep) {
//         thread::sleep(Duration::from_secs(5));
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else {
//         ("HTTP/1.1 2404 NOT FOUND", "404.html")
//     };

//     let mut file = File::open(filename).unwrap();
//     let mut contents = String::new();

//     file.read_to_string(&mut contents).unwrap();

//     let response  = format!("{}\r\nContents-Length: {}\r\n\r\n{}",
//         status_line,
//         contents.len(),
//         contents
//     );

//     stream.write(&response.as_bytes()).unwrap();
//     stream.flush().unwrap();

// }

// pub struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }

// impl ThreadPool {
//     fn new(size: usize) -> Result<Self, ZeroWorkerError> {
//         if size == 0 {
//             return Err(ZeroWorkerError);
//         }

//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc::new(Mutex::new(receiver));

//         let mut workers = Vec::with_capacity(size);
//         for id in 0..size {
//             workers.push(Worker::new(id, receiver.clone()));
//         }

//         Ok(
//             Self {
//                 workers,
//                 sender,
//             }
//         )
//     }

//     pub fn execute<F>(&self, func: F) 
//         where 
//             F: FnOnce() + Send + 'static 
//     {
//         let job = Box::new(func);
//         self.sender.send(Message::NewJob(job)).unwrap();
//     }
// }

// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>
// }

// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {

//         let thread = thread::spawn(move || {
//             loop {
//                 let message = receiver.lock().unwrap().recv().unwrap();
//                 match message {
//                     Message::NewJob(job) => {
//                         #[allow(unused_variables)]
//                         let thread = thread::spawn(move || {
//                             println!("{} worker got a job", id);
                                
//                             job.call_box();
//                     });
//                     },
//                     Message::Terminate => {
//                         println!("Shutting down");
//                         break;
//                     },
//                 }
        
//             }            
//         });

//         Self {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         println!("Sendding terminate message to all worekrs.");

//         for _ in &mut self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         println!("The last job to terminiate.");
        
//         for worker in &mut self.workers {
//             println!("{} worker - terminiating..", worker.id);

//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

// enum Message {
//     NewJob(Job),
//     Terminate,
// }

// type Job = Box<dyn FnBox + Send + 'static>;

// trait FnBox {
//     fn call_box(self: Box<Self>);
// }

// impl<F: FnOnce()> FnBox for F {
//     fn call_box(self: Box<Self>) {
//         (*self)()
//     }
// }

// struct ZeroWorkerError;

// impl Display for ZeroWorkerError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self)
//     }
// }

// impl Debug for ZeroWorkerError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Size shouble not be zero.")
//     }
// }

// impl std::error::Error for ZeroWorkerError {}