use crate::cli::Args;

#[derive(Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl PostgresConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug)]
pub struct ProxyConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug)]
pub struct Config {
    pub proxy: ProxyConfig,
    pub postgres: PostgresConfig,
}

pub fn load_config(args: Args) -> Config {
    let proxy = ProxyConfig {
        port: args.proxy.port,
        host: args.proxy.host,
    };

    let postgres = PostgresConfig {
        host: args.postgres.host,
        port: args.postgres.port,
        user: args.postgres.user,
        password: args.postgres.password,
        database: args.postgres.database,
    };

    Config { proxy, postgres }
}
