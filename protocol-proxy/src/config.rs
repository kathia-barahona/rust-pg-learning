use crate::cli::Args;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_port: u16,
    pub proxy_port: u16,
}

pub fn load_config(args: Args) -> Config {
    let server_port = args
        .server_port
        .or_else(|| env::var("SERVER_PORT").ok().and_then(|v| v.parse().ok()))
        .unwrap_or(5050);

    let proxy_port = args
        .proxy_port
        .or_else(|| env::var("PROXY_PORT").ok().and_then(|v| v.parse().ok()))
        .unwrap_or(5000);

    Config {
        server_port,
        proxy_port,
    }
}
