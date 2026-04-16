use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub server_port: Option<u16>,

    #[arg(long)]
    pub proxy_port: Option<u16>,
}
