use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use clap::Parser;

mod cli;
mod config;

fn start_proxy(port: u16, server_port: u16) -> std::io::Result<()> {
    for stream in listen_on(port).incoming() {
        proxy_handle_client(stream?, server_port);
    }
    Ok(())
}

fn start_server(port: u16) -> std::io::Result<()> {
    for stream in listen_on(port).incoming() {
        server_handle_client(stream?);
    }
    Ok(())
}

fn proxy_handle_client(mut stream: TcpStream, server_port: u16) {
    println!("Connected to {}", stream.peer_addr().unwrap());
    let buffer_size: usize = 1024;
    let mut buffer = vec![0u8; buffer_size];

    let mut server_buffer = vec![0u8; buffer_size];
    let mut server_stream = TcpStream::connect(("127.0.0.1", server_port)).unwrap();

    loop {
        // only write back what it was read
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            println!("Client disconnected");
            break;
        }

        server_stream.write_all(&buffer[..n]).unwrap();
        let n = server_stream.read(&mut server_buffer).unwrap();

        if n == 0 {
            println!("Server disconnected");
            stream
                .write_all(b"Something wrong occurred, server was disconnected!")
                .unwrap();
            break;
        }
        stream.write_all(&server_buffer[..n]).unwrap();
    }
}

fn server_handle_client(mut stream: TcpStream) {
    let buffer_size: usize = 1024;
    let mut buffer = vec![0u8; buffer_size];
    loop {
        // only write back what it was read
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            println!("Client disconnected");
            break;
        }
        println!(
            "Received message! {}",
            str::from_utf8(&buffer[..n]).unwrap()
        );
        stream.write_all(&buffer[..n]).unwrap();
    }
}

fn listen_on(port: u16) -> TcpListener {
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    listener
}

fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let config = config::load_config(args);

    println!("{:?}", config);

    let server_handle = thread::spawn(move || {
        let _ = start_server(config.server_port);
    });
    // wait for server to start
    thread::sleep(std::time::Duration::from_millis(100));
    let proxy_handle = thread::spawn(move || {
        let _ = start_proxy(config.proxy_port, config.server_port);
    });

    server_handle.join().unwrap();
    proxy_handle.join().unwrap();

    Ok(())
}
