use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

// Asynchronous version
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        println!("Accepted new connection");
        tokio::spawn(async move {
            process_socket(stream).await;
        });
    }
}

async fn process_socket(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(_bytes) => {
                stream
                    .write_all(b"+PONG\r\n")
                    .await
                    .expect("Could not write to buffer");
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
    }
}
