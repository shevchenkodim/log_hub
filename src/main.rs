mod adapters;
mod application;
mod domain;
mod infrastructure;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::AppConfig;

use std::sync::Arc;

use crate::adapters::spi::postgres::connection::PostgresClient;
use crate::infrastructure::config::Configuration;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Configuration::make_from_env();

    let postgres_client: PostgresClient = PostgresClient::new(config.postgresql_config()).await;
    // rabbimq & redis

    // let repositories = Repositories::new();
    // let services = Services::new(repositories);

    let config_arc: Arc<Configuration> = Arc::new(config.clone());
    let postgres_client_arc: Arc<PostgresClient> = Arc::new(postgres_client);

    HttpServer::new(move || {
        App::new()
            .app_data(config_arc.clone())
            .app_data(postgres_client_arc.clone())
    })
        .bind(config.http_server_url)?
        .run()
        .await
}
