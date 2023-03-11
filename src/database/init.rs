use sea_orm::{ConnectionTrait, Database, DbErr, Statement};

const DB_NAME: &str = "password_manager";
// const DATABASE_URL: &str = "postgres://abdulqudduus:andromeda@localhost/password_manager";
const DATABASE_URL: &str = "postgres://abdulqudduus:andromeda@localhost/password_manager";

pub async fn init() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    // let db = db.get_database_backend();
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
    )).await?;

    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", DB_NAME),
    )).await?;

    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    
    Database::connect(&url).await?;

    Ok(())
}
