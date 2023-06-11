use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use log::error;
use migration::DbErr;
use sea_orm::TransactionError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: serde_json::Value,
}

impl ApiError {
    pub fn new(status_code: u16, message: serde_json::Value) -> ApiError {
        ApiError {
            status_code,
            message,
        }
    }

    pub fn unauthorized() -> ApiError {
        ApiError::new(401, json!("This request is unauthorized".to_string()))
    }

    pub fn internal_server_error() -> ApiError {
        ApiError::new(
            500,
            serde_json::Value::String("Internal server error".to_string()),
        )
    }

    pub fn bad_request(message: serde_json::Value) -> ApiError {
        ApiError::new(400, message)
    }

    // pub fn bad_request_json<T: Serialize>(message: T) -> ApiError {
    //     ApiError::new(400, message)
    // }

    pub fn not_found(message: serde_json::Value) -> ApiError {
        ApiError::new(404, message)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message.to_string())
    }
}

impl<T: std::error::Error> std::convert::From<TransactionError<T>> for ApiError {
    fn from(value: TransactionError<T>) -> Self {
        error!("Transaction error: {}", value.to_string());
        ApiError::internal_server_error()
    }
}

impl From<DbErr> for ApiError {
    fn from(error: DbErr) -> Self {
        match error {
            DbErr::ConnectionAcquire => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::TryIntoErr {
                from: _,
                into: _,
                source: _,
            } => ApiError::new(500, serde_json::Value::String("Diesel error: ".to_string())),
            DbErr::Conn(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::Exec(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::Query(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::ConvertFromU64(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::UnpackInsertId => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::UpdateGetPrimaryKey => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::RecordNotFound(_) => ApiError::new(
                404,
                serde_json::Value::String("Record not found".to_string()),
            ),
            DbErr::AttrNotSet(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::Custom(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::Type(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::Json(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::Migration(_) => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::RecordNotInserted => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
            DbErr::RecordNotUpdated => ApiError::new(
                500,
                serde_json::Value::String(format!("Diesel error: {}", error)),
            ),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                error!("{:?}", self.message);
                serde_json::Value::String("Internal Server Error".to_string())
            }
        };

        HttpResponse::build(status_code).json(json!(message))
    }
}
