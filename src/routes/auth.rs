use std::collections::BTreeMap;

use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidateArgs};

use crate::{
    api_error::ApiError,
    dto::auth::{create_user::CreateUserDTO, login::LoginDTO},
    entities::users::Model as User,
    services::auth::sign,
    AppState,
};

#[post("/register")]
async fn register(
    dto: Json<CreateUserDTO>,
    data: Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let conn = &data.conn;

    match dto.validate_args(("email", "users", conn)) {
        Ok(_) => (),
        Err(e) => {
            return Err(ApiError::bad_request(json!({ "message": e })));
        }
    }

    let result = User::create(dto.into_inner(), conn).await?.unwrap();

    Ok(HttpResponse::Created().json(json!({ "data": result })))
}

#[post("/login")]
async fn login(dto: Json<LoginDTO>, data: Data<AppState>) -> Result<HttpResponse, ApiError> {
    let conn = &data.conn;

    match dto.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(ApiError::bad_request(json!({ "message": e })));
        }
    }

    let email = dto.email.as_ref().unwrap();

    let result = User::find_by_email(email, conn).await?;

    if let Some(user) = result {
        if !user
            .verify_password(dto.password.as_ref().unwrap())
            .unwrap()
        {
            return Err(ApiError::unauthorized());
        }

        let user_id = user.id.to_string();
        let name = user.name.to_string();
        let email = user.email.to_string();

        let mut claims = BTreeMap::new();
        claims.insert("id", user_id.as_str());
        claims.insert("name", name.as_str());
        claims.insert("email", email.as_str());

        let token = sign(claims).map_err(|_| ApiError::internal_server_error());

        Ok(HttpResponse::Ok().json(LoginResponse {
            token: token.unwrap(),
        }))
    } else {
        Err(ApiError::unauthorized())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
