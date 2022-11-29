use rand::Rng;
use std::{
    io::prelude::{Read, Write},
    net::TcpStream,
    str::from_utf8,
    thread, time,
};

fn main() {
    let mut rng_num = rand::thread_rng();
    let address = "127.0.0.1:5001".to_owned();

    for _ in 0..10 {
        let mut stream = TcpStream::connect(&address).unwrap();
        let one_sec = time::Duration::from_secs(1);
    
        thread::sleep(one_sec);
        let num = rng_num.gen_range(1..11).to_string();

        stream.write(num.as_bytes()).unwrap();
        println!("Send : [{} - {}]", &address, &num);

        let mut buffer: [u8; 64] = [0; 64];
        let _ = stream.read(&mut buffer).unwrap();

        let result = from_utf8(&buffer).unwrap();
        println!("Result : {}", result);
    }
}
