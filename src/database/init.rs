use std::env;

use log::info;
use sea_orm::{ConnectionTrait, Database, DbErr, Statement, DatabaseConnection};

pub async fn init() -> Result<DatabaseConnection, DbErr> {
    info!("Connecting to database...");
    
    let database_url = env::var("DATABASE_URL").unwrap();
    let db = Database::connect(&database_url).await?;

    // let db = db.get_database_backend();
    // db.execute(Statement::from_string(
    //     db.get_database_backend(),
    //     format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
    // )).await?;

    // db.execute(Statement::from_string(
    //     db.get_database_backend(),
    //     format!("CREATE DATABASE \"{}\";", DB_NAME),
    // )).await?;

    let url = format!("{}", database_url);
    
    Database::connect(&url).await?;
    info!("Connected to database");

    Ok(db)
}
