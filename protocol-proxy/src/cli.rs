use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(flatten)]
    pub proxy: ProxyArgs,

    #[command(flatten)]
    pub postgres: PostgresArgs,
}

#[derive(Parser, Debug)]
pub struct ProxyArgs {
    #[clap(
        id = "proxy_port",
        long = "proxy-port",
        env = "PROXY_PORT",
        default_value_t = 5000
    )]
    pub port: u16,

    #[clap(
        id = "proxy_host",
        long = "proxy-host",
        env = "PROXY_HOST",
        default_value = "0.0.0.0"
    )]
    pub host: String,
}

#[derive(Parser, Debug)]
pub struct PostgresArgs {
    #[arg(
        id = "pg_host",
        long = "pg-host",
        env = "PG_HOST",
        default_value = "localhost"
    )]
    pub host: String,

    #[arg(
        id = "pg_port",
        long = "pg-port",
        env = "PG_PORT",
        default_value_t = 5432
    )]
    pub port: u16,

    #[arg(long = "pg-user", env = "PG_USER", default_value = "postgres")]
    pub user: String,

    #[arg(long = "pg-password", env = "PG_PASSWORD", default_value = "")]
    pub password: String,

    #[arg(long = "pg-database", env = "PG_DATABASE", default_value = "postgres")]
    pub database: String,
}
