use futures::executor::block_on;
use sea_orm::{DbBackend, DbConn, FromQueryResult, JsonValue, Statement};
use validator::ValidationError;

async fn unique_inner(value: &str, arg: (&str, &str, &DbConn)) -> Result<(), ValidationError> {
    let (field, table, conn) = arg;

    let query = format!(
        "SELECT * FROM \"{}\" WHERE {} = '{}' ORDER BY id LIMIT 1",
        table, field, value
    );

    let result: Vec<JsonValue> = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        &query,
        [],
    ))
    .all(conn)
    .await
    .unwrap();

    if !result.is_empty() {
        return Err(ValidationError::new("Unique constraint error"));
    }

    Ok(())
}

pub fn unique(value: &str, arg: (&str, &str, &DbConn)) -> Result<(), ValidationError> {
    return block_on(unique_inner(value, arg));
}
