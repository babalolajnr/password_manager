use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use log::error;
use migration::DbErr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: ErrorMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrorMessage {
    Text(String),
    Json(serde_json::Value),
}

impl ApiError {
    pub fn new(status_code: u16, message: ErrorMessage) -> ApiError {
        ApiError {
            status_code,
            message,
        }
    }

    pub fn unauthorized() -> ApiError {
        ApiError::new(
            401,
            ErrorMessage::Json(json!("This request is unauthorized".to_string())),
        )
    }

    pub fn internal_server_error() -> ApiError {
        ApiError::new(500, ErrorMessage::Text("Internal server error".to_string()))
    }

    pub fn bad_request(message: ErrorMessage) -> ApiError {
        ApiError::new(400, message)
    }

    // pub fn bad_request_json<T: Serialize>(message: T) -> ApiError {
    //     ApiError::new(400, message)
    // }

    pub fn not_found(message: ErrorMessage) -> ApiError {
        ApiError::new(404, message)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            ErrorMessage::Text(message) => f.write_str(message.as_str()),
            ErrorMessage::Json(message) => f.write_str(&message.to_string()),
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(error: DbErr) -> Self {
        match error {
            DbErr::ConnectionAcquire => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::TryIntoErr {
                from: _,
                into: _,
                source: _,
            } => ApiError::new(500, ErrorMessage::Text(format!("Diesel error: "))),
            DbErr::Conn(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::Exec(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::Query(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::ConvertFromU64(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::UnpackInsertId => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::UpdateGetPrimaryKey => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::RecordNotFound(_) => {
                ApiError::new(404, ErrorMessage::Text("Record not found".to_string()))
            }
            DbErr::AttrNotSet(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::Custom(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::Type(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::Json(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::Migration(_) => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::RecordNotInserted => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
            DbErr::RecordNotUpdated => ApiError::new(
                500,
                ErrorMessage::Text(format!("Diesel error: {}", error.to_string())),
            ),
        }
    }
}

// impl From<DieselError> for ApiError {
//     fn from(error: DieselError) -> ApiError {
//         match error {
//             DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
//             DieselError::NotFound => ApiError::new(404, "Record not found".to_string()),
//             err => ApiError::new(500, format!("Diesel error: {}", err)),
//         }
//     }
// }

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
                ErrorMessage::Text("Internal Server Error".to_string())
            }
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
