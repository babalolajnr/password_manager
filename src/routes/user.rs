use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::api_error::ApiError;

#[get("/users")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    // let users = User::find_all()?;
    // Ok(HttpResponse::Ok().json(users))
    todo!()

}

#[get("/users/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    todo!()

    // let user = User::find(id.into_inner())?;
    // Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create() -> Result<HttpResponse, ApiError> {
    // let user = User::create(user.into_inner())?;
    // Ok(HttpResponse::Ok().json(user))
    todo!()

}

#[put("/users/{id}")]
async fn update(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    todo!()
    
    // let user = User::update(id.into_inner(), user.into_inner())?;
    // Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    todo!()
    // let num_deleted = User::delete(id.into_inner())?;
    // Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
