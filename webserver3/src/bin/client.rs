use std::{
    io::{Read, Write, stdin},
    net::TcpStream,
    sync::{mpsc},
    str::from_utf8,
    thread,
};

fn main() {

    let mut stream = TcpStream::connect("172.30.1.2:50001").unwrap();

    loop {
        let mut stream_clone = stream.try_clone().unwrap();

        println!("input message:");
        thread::spawn(move || {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            
            stream_clone.write(&buffer.as_bytes()).unwrap();
    
        });
    
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).unwrap();
    
        println!("{}", from_utf8(&buffer).unwrap());
    }
}


// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let mut stream = TcpStream::connect("127.0.0.1:50000").unwrap();
//     let mut stream_send = stream.try_clone().unwrap();

//     thread::spawn(move || {
//         show(rx);
//     });

//     thread::spawn(move || {
//         input(stream_send);
//     });

//     loop {
//         let mut buffer: [u8; 1024] = [0; 1024];
//         stream.read(&mut buffer).unwrap();
//         tx.send(buffer).unwrap();
//     }
// }


// fn show(rx: mpsc::Receiver<[u8; 1024]>) {
//     loop {
//         let read_bytes = rx.recv().unwrap();
//         println!("{}", from_utf8(&read_bytes).unwrap());
//     }
// }

// fn input(mut stream: TcpStream) {
//     loop {
//         let mut buffer = String::new();
//         stdin().read_line(&mut buffer).unwrap();
    
//         stream.write(buffer.as_bytes()).unwrap();
//     }
// }