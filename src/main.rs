use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Failed to read from stream");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Recieved request {}", request);

    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!\n");
    stream
        .write(response.as_bytes())
        .expect("Failed to write to stream");
}

fn main() {
    let host = "127.0.0.1:8080";
    let listener = TcpListener::bind(host).expect("Failed to bind to address");

    println!("Server listening on {}", host);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
