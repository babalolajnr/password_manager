use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use serde_json::json;
use validator::Validate;

use crate::{
    api_error::ApiError,
    dto::password::create_password::CreatePasswordDTO,
    entities::{passwords::Model as Password, users::Model as User},
    AppState,
};

#[post("/")]
async fn create_password(
    dto: Json<CreatePasswordDTO>,
    data: Data<AppState>,
    user: User,
) -> Result<HttpResponse, ApiError> {
    let conn = &data.conn;

    match dto.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(ApiError::bad_request(json!({ "message": e })));
        }
    }

    let password = CreatePasswordDTO::new(&dto, &user.id);
    let result = Password::create(password.into(), conn).await?.unwrap();
    Ok(HttpResponse::Ok().json(json!({ "data": result })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_password);
}
