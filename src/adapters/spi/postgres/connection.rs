use deadpool_postgres::{Client, Config, ManagerConfig, Pool, RecyclingMethod, SslMode};
use deadpool_postgres::tokio_postgres::NoTls;
use crate::adapters::spi::postgres::migrations::run_migrations;


#[derive(Debug)]
pub struct PostgresClientConfig {
    pub db_name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub ssl: SslMode,
}

#[derive(Clone)]
pub struct PostgresClient {
    pg_config: Config,
    pg_pool: Option<Pool>,
}

impl PostgresClient {
    pub async fn new(pg_client_config: PostgresClientConfig) -> PostgresClient {
        // make postgres config
        let mut config = Config::new();

        config.dbname = Some(pg_client_config.db_name);
        config.host = Some(pg_client_config.host);
        config.port = Some(pg_client_config.port);
        config.user = Some(pg_client_config.user);
        config.password = Some(pg_client_config.password);
        config.ssl_mode = Some(pg_client_config.ssl);

        config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

        // make postgres client
        let mut pg_client: PostgresClient = PostgresClient {
            pg_config: config,
            pg_pool: None
        };

        pg_client.pg_pool = Some(pg_client.create_pool());

        // run migrations
        pg_client.run_migrations().await;

        pg_client
    }

    pub fn create_pool(&self) -> Pool {
        match self.pg_config.create_pool(None, NoTls) {
            Ok(p) => { p },
            Err(err) => { panic!("{}", err.to_string()) }
        }
    }

    pub async fn get_connection(&self) -> Client {
        match &self.pg_pool {
            None => {
                panic!("Connection pool is None!")
            },
            Some(pool) => {
                pool.get().await.expect("Can't get connection from pool!")
            }
        }
    }

    pub async fn run_migrations(&self) {
        run_migrations(self).await;
    }
}
