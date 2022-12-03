use tokio::{
    net::{TcpListener, TcpStream},
    time::{sleep, Duration},
    io::{AsyncReadExt, AsyncWriteExt},
};    

#[tokio::main(flavor = "multi_thread", worker_threads = 128)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("172.30.1.2:50000").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        let mut buf:[u8; 10] = [0; 10];

        tokio::spawn(async move {
            loop {
                stream.read(&mut buf).await.unwrap();

                // tokio::spawn(async move {
                //     let mut db_buf: [u8; 10] = [1; 10];
                //     let mut db_stream = TcpStream::connect("172.30.1.2:5000").await.unwrap();
                //     db_stream.write(&db_buf).await.unwrap();
                //     db_stream.read(&db_buf).await.unwrap();
        
                // });

                stream.write(&buf).await.unwrap();
            }
        });
    }
    Ok(())
}
