use std::{
    collections::HashSet,
    io::prelude::{Read, Write},
    net::TcpListener,
    str::from_utf8,
    sync::{Arc, Mutex},
    thread,
};

fn main() -> std::io::Result<()> {
    let db = Arc::new(Mutex::new(HashSet::<String>::new()));
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    loop {
        match listener.accept() {
            Ok((mut socket, _addr)) => {
                let db_clone = db.clone();
                thread::spawn(move || {
                    let mut buffer: [u8; 64] = [0; 64];
                    let _ = socket.read(&mut buffer).unwrap();
                    let buf_str = from_utf8(&buffer).unwrap().to_owned();
                    let mut result = true;

                    {
                        let mut db_guard = db_clone.lock().unwrap();
                        // println!("{}", &buf_str);
                        result = db_guard.insert(buf_str);
                    }

                    match result {
                        true => socket.write("Ok".as_bytes()).unwrap(),
                        false => socket.write("Denied".as_bytes()).unwrap(),
                    }
                });
            }
            Err(e) => println!("{}", e),
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
