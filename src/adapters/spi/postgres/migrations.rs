use std::ops::DerefMut;
use deadpool_postgres::{Client};
use crate::adapters::spi::postgres::connection::PostgresClient;


mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn run_migrations(db: &PostgresClient) {
    let mut conn: Client = db.get_connection().await;
    let client = conn.deref_mut().deref_mut();
    match embedded::migrations::runner().run_async(client).await {
        Ok(_) => {
            println!("Migrations successfully applied!")
        }
        Err(err) => {
            panic!("No migrations took place! Error: {}", err);
        }
    }
}
