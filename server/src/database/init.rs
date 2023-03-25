use std::env;

use log::info;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn init() -> Result<DatabaseConnection, DbErr> {
    info!("Connecting to database...");

    let database_url = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(&database_url).await?;

    let url = format!("{}", database_url);

    Database::connect(&url).await?;
    info!("Connected to database");

    Ok(db)
}
