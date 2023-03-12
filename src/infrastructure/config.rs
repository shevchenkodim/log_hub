use deadpool_postgres::SslMode;


#[derive(Debug, Clone)]
pub struct Configuration {
    // system
    pub debug: bool,

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
}
