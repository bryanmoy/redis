// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::with_capacity(512);

    loop {
        let mut reader = BufReader::new(&stream);

        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_bytes) => {
                stream
                    .write_all(b"+PONG\r\n")
                    .expect("Failed to write to client");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
