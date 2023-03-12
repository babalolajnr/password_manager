use chrono::Utc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{entities::user::Model};

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(required(message = "Name is required"))]
    pub name: Option<String>,

    #[validate(
        required(message = "Email is required"),
        email(message = "Invalid email format"),
        length(
            min = 3,
            message = "Email is too short. It must be at least 3 characters long"
        )
    )]
    pub email: Option<String>,

    #[validate(
        required(message = "Password is required"),
        length(
            min = 8,
            message = "Password is too short. It must be at least 8 characters long"
        )
    )]
    pub password: Option<String>,
}

impl From<CreateUserDTO> for Model {
    fn from(user: CreateUserDTO) -> Self {
        let timestamp = Utc::now().naive_utc();

        Model {
            id: Uuid::new_v4(),
            name: user.name.unwrap(),
            email: user.email.unwrap(),
            password: bcrypt::hash(&user.password.unwrap(), 4).unwrap(),
            created_at: timestamp,
            updated_at: timestamp,
            deleted_at: None,
        }
    }
}

// impl FromRequest for CreateUserDTO {
//     type Error = ApiError;

//     type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

//     fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
//         todo!()
//     }


// }
