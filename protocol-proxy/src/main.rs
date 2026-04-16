use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};

fn handle_client(mut stream: TcpStream) {
    println!("Connected to {}", stream.peer_addr().unwrap());
    let buffer_size: usize = 1024;
    let mut buffer = vec![0u8; buffer_size];
    loop {
        // only write back what it was read
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            println!("Client disconnected");
            break;
        }
        stream.write_all(&buffer[..n]).unwrap();
    }
}

fn listen_on(port: u16) -> TcpListener{
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    listener
}
fn main() -> std::io::Result<()> {
    let port: u16 = 5000;
    // accept connections and process them serially
    for stream in listen_on(port).incoming() {
        handle_client(stream?);
    }
    Ok(())
}
