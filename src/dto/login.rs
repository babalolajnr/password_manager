use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(required(message = "Email is required"))]
    pub email: Option<String>,

    #[validate(required(message = "Password is required"))]
    pub password: Option<String>,
}

