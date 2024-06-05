use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // buffer to read data from the client
    let mut buffer = [0; 1024];
    // read data from the stream and store it in the buffer
    stream
        .read(&mut buffer)
        .expect("Failed to read from client!");
    // converts the data in the buffer into a UTF-8 encoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response!");
}

fn main() {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).expect("Failed to bind to address: {address}");
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}!", e);
            }
        }
    }
}
