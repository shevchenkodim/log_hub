use deadpool_postgres::SslMode;
use serde::Deserialize;
use crate::adapters::spi::postgres::connection::PostgresClientConfig;


#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    // system
    pub debug: bool,
    pub http_server_url: String,

    // basic authentication
    pub system_username: String,
    pub system_password: String,

    // postgres
    pub database_name: String,
    pub database_host: String,
    pub database_port: u16,
    pub database_user: String,
    pub database_password: String,

    // redis
    // ...

    // mongodb
    // ...
}

impl Configuration {
    pub fn make_from_env() -> Configuration {
         envy::from_env::<Configuration>().expect("Error on loading environment variables")
    }

    pub fn database_ssl_mode(&self) -> SslMode {
        let mut ssl_mode = SslMode::Require;
        if self.debug {
            ssl_mode = SslMode::Disable;
        }
        ssl_mode
    }

    pub fn postgresql_config(&self) -> PostgresClientConfig {
        PostgresClientConfig {
            db_name: self.database_name.to_string(),
            host: self.database_host.to_string(),
            port: self.database_port,
            user: self.database_user.to_string(),
            password: self.database_password.to_string(),
            ssl: self.database_ssl_mode(),
        }
    }
}
