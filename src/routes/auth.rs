use actix_web::{
    post,
    web::{self, Json, Data},
    HttpResponse,
};
use sea_orm::DbConn;
use serde_json::json;
use validator::Validate;

use crate::{api_error::{ApiError, ErrorMessage}, dto::create_user::CreateUserDTO, entities::user::Model, AppState};

#[post("/register")]
async fn register(dto: Json<CreateUserDTO>, data: Data<AppState>) -> Result<HttpResponse, ApiError> {
    let conn = &data.conn;

    match dto.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(ApiError::bad_request(ErrorMessage::Json(json!({"message": e}))));
        }
    }

    let result = Model::create(dto.into_inner(), conn).await?;

    // Ok(HttpResponse::Ok().json(result))
    Ok(HttpResponse::Ok().json(json!({"status": "created"})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}
