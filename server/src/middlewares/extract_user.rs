use actix_web::{web::Data, FromRequest};
use futures::{
    future::{ready, Either, LocalBoxFuture, Ready},
    FutureExt,
};
use log::error;
use serde_json::json;
use uuid::Uuid;

use crate::{
    api_error::ApiError, entities::users::Model as User, services::auth::verify, AppState,
};

impl FromRequest for User {
    type Error = ApiError;
    type Future = Either<
        Ready<Result<Self, Self::Error>>,
        LocalBoxFuture<'static, Result<Self, Self::Error>>,
    >;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let authorization = req
            .headers()
            .get("Authorization")
            .map(|header| header.to_str().map(|s| s.to_owned()));

        match authorization {
            Some(Ok(token)) => {
                let verify = verify(&token);

                match verify {
                    Ok(claims) => {
                        let app_state = req.app_data::<Data<AppState>>().unwrap().clone();

                        let uuid = Uuid::parse_str(claims.get("id").unwrap())
                            .map_err(|e| {
                                error!("{}", e);
                                ApiError::unauthorized()
                            })
                            .unwrap();

                        Either::Right(
                            async move {
                                User::find(&uuid, &app_state.conn).await?.ok_or_else(|| {
                                    error!("User not found");
                                    ApiError::not_found(json!("User not found".to_string()))
                                })
                            }
                            .boxed_local(),
                        )
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Either::Left(ready(Err(ApiError::unauthorized())))
                    }
                }
            }
            Some(Err(_)) => {
                error!("Token is not a valid string");
                Either::Left(ready(Err(ApiError::unauthorized())))
            }
            None => {
                error!("No token provided");
                Either::Left(ready(Err(ApiError::unauthorized())))
            }
        }
    }
}
