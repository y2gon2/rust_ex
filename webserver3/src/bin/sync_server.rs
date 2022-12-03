use std::{
    io::{Write, Read},
    thread,
    net::{TcpListener, TcpStream},
    time::Duration,
    path::Path,
};

fn main() {
    let listener = TcpListener::bind("172.30.1.2:50001").unwrap();

    loop {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf:[u8; 10] = [0; 10];
        thread::spawn(move || {
            loop {
                stream.read(&mut buf).unwrap();
               
                
                // thread::spawn(move || {
                //     let mut db_buf: [u8; 10] = [1; 10];
                //     let mut db_stream = TcpStream::connect("172.30.1.2:5000").unwrap();
                //     db_stream.write(&db_buf).unwrap();
                //     db_stream.read(&mut db_buf).unwrap(); 
                // });

                stream.write(&buf).unwrap();
            }
        });
    }

}
