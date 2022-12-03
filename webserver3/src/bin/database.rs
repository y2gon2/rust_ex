use std::{
    collections::HashSet,
    io::prelude::{Read, Write},
    net::TcpListener,
    str::from_utf8,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let db = Arc::new(Mutex::new(HashSet::<String>::new()));
    let listener = TcpListener::bind("172.30.1.2:5000").unwrap();

    loop {
        match listener.accept() {
            Ok((mut socket, _addr)) => {
                let mut buf: [u8; 10] = [0; 10];
                socket.read(&mut buf).unwrap();

                thread::sleep(Duration::from_millis(1000));

                socket.write(&buf).unwrap();
            }
            Err(e) => println!("{}", e),
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
