use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use zxcvbn::zxcvbn;

use crate::entities::passwords::Model as Password;

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct CreatePasswordDTO {
    #[validate(
        required(message = "URL is required"),
        url(message = "Provide a valid URL")
    )]
    pub url: Option<String>,

    pub website: Option<String>,

    #[validate(required(message = "Username is required"))]
    pub username: Option<String>,

    #[validate(required(message = "Password is required"))]
    pub password: Option<String>,

    pub tags: Option<String>,

    pub user_id: Option<Uuid>,
}

impl CreatePasswordDTO {
    pub fn new(dto: &CreatePasswordDTO, user_id: &Uuid) -> Self {
        let pass_dto = dto.to_owned();

        CreatePasswordDTO {
            url: pass_dto.url,
            website: pass_dto.website,
            username: pass_dto.username,
            password: pass_dto.password,
            tags: pass_dto.tags,
            user_id: Some(user_id.to_owned()),
        }
    }
}

impl From<CreatePasswordDTO> for Password {
    fn from(password: CreatePasswordDTO) -> Self {
        let timestamp = Utc::now().naive_utc();

        let password_value = password.password.unwrap();
        let username = password.username.unwrap();
        let website = password.website.unwrap();
        
        // Calculate strength of password
        let strength = zxcvbn(&password_value, &[&username, &website]).unwrap();

        Password {
            id: Uuid::new_v4(),
            user_id: password.user_id.unwrap(),
            website: Some(website),
            username,
            password: password_value,
            note: None,
            tags: password.tags,
            url: password.url.unwrap(),
            security_question: None,
            created_at: timestamp,
            updated_at: timestamp,
            deleted_at: None,
            expired_at: None,
            strength: strength.score() as i32,
        }
    }
}
