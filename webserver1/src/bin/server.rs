use std::{
    io::{prelude::Write, Read, stdin},
    net::{Shutdown, TcpListener, TcpStream},
    thread,
    sync::{Arc, Mutex},
};

fn main() -> std::io::Result<()> {
    let counter = Arc::new(Mutex::<u8>::new(0));

    monitor(counter.clone());
    service(counter.clone());
    
    Ok(())
}

fn monitor(counter: Arc<Mutex<u8>>) {
    thread::spawn(move || {
        loop {
            println!("Please press the number to check");
            println!("1. current connected client number");
            let mut buffer = String::new();
            let _ = stdin().read_line(&mut buffer).unwrap();
            if buffer.trim() == "1".to_owned() {
                println!("current connected client number : {}", *counter.lock().unwrap());
            }
        }
    });
}

fn service(counter: Arc<Mutex<u8>>) {
    let user_listener = TcpListener::bind("127.0.0.1:5001").unwrap();

    loop {
        let mut temp: u8 = 0;
        {
            let mut counter_guard = counter.lock().unwrap();
            temp = *counter_guard;
        }
        match temp < 2 {
            true => {
                let counter_clone = counter.clone();
                let (mut user_socket, _addr) = user_listener.accept().unwrap();
                {
                    let mut counter_guard = counter.lock().unwrap();
                    *counter_guard += 1;
                }
                thread::spawn(move || {
                    loop {
                        let mut buffer: [u8; 64] = [0; 64];
                        match user_socket.read(&mut buffer) {
                            Ok(_a) => {
                                let mut db_socket = TcpStream::connect("127.0.0.1:5000").unwrap();
                                db_socket.write(&buffer).unwrap();
        
                                let mut result_buffer: [u8; 64] = [0; 64];
                                let _ = db_socket.read(&mut result_buffer).unwrap();
                                
                                user_socket.write(&result_buffer);
                            }, 
                            Err(_) => {
                                // println!("Disconnected");
                                {
                                    let mut counter_clone_guard = counter_clone.lock().unwrap();
                                    *counter_clone_guard -= 1; 
                                    // println!("{}", *counter_clone_guard);
                                }
                                break;
                            }
                        }
                    }
                });
    
            }
            false => {
                let (user_socket, _addr) = user_listener.accept().unwrap();
                user_socket
                    .shutdown(Shutdown::Both)
                    .expect("shutdown call failed");
            }
        }
    }
}

