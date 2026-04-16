use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use clap::Parser;

mod cli;
mod config;

fn start_proxy(config: &config::Config) -> std::io::Result<()> {
    for stream in listen_on(config.proxy.host.clone(), config.proxy.port).incoming() {
        proxy_handle_client(stream?, &config);
    }
    Ok(())
}

fn proxy_handle_client(mut client_stream: TcpStream, config: &config::Config) {
    println!(
        "Incoming connection from {}",
        client_stream.peer_addr().unwrap()
    );
    // TODO: shouldn't use a fixed buffer size, should resize based on request
    let buffer_size: usize = 1024;
    let mut client_buffer = vec![0u8; buffer_size];

    let mut conn_attempts = 3;
    let mut pg_buffer = vec![0u8; buffer_size];
    let mut pg_stream = loop {
        match TcpStream::connect(config.postgres.address()) {
            Ok(stream) => break stream,
            Err(_) if conn_attempts > 1 => {
                conn_attempts -= 1;
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            Err(e) => {
                println!("Failed to connect to postgres database: {}", e);
                let _ = client_stream.write_all(b"Failed to connect to postgres database.");
                return;
            }
        }
    };
    loop {
        // only write back what it was read
        let n = client_stream.read(&mut client_buffer).unwrap();
        if n == 0 {
            println!("Client disconnected");
            break;
        }

        println!(
            "Client -> PG ({} bytes): {:02x?}",
            n,
            &client_buffer[..n.min(64)]
        );

        pg_stream.write_all(&client_buffer[..n]).unwrap();
        let n = pg_stream.read(&mut pg_buffer).unwrap();

        if n == 0 {
            println!("Client/Server disconnected");
            client_stream
                .write_all(b"Something wrong occurred, server was disconnected!")
                .unwrap();
            break;
        }
        println!(
            "PG -> Client ({} bytes): {:02x?}",
            n,
            &pg_buffer[..n.min(64)]
        );
        client_stream.write_all(&pg_buffer[..n]).unwrap();
    }
}

fn listen_on(host: String, port: u16) -> TcpListener {
    let listener = TcpListener::bind((host, port)).unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    listener
}

fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    let config = config::load_config(args);

    println!("{:?}", config);
    let _ = start_proxy(&config);

    Ok(())
}
