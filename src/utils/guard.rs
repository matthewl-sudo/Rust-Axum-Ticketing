// use axum::{
//     Extension,
//     extract::State,
//     http::{ Request, StatusCode}, 
//     middleware::Next, response::Response, 
// };
// use axum_extra::{
//     headers::{authorization::Bearer, Authorization, HeaderMapExt},
//     TypedHeader,
//     extract,
// };
// // use axum_extra::headers::{HeaderMapExt, Authorization, authorization::Bearer}
// use super::jwt::decode_jwt;
// use sqlx::MySqlPool;
// use crate::{error::AppError, AppState}; 


// pub async fn guard<T>(
//     mut req: Request<T>, next: Next,Extension(pool): Extension<MySqlPool>) -> Result<Response, AppError> {

//     let token = req.headers().typed_get::<Authorization<Bearer>>()
//     .ok_or(AppError::InvalidToken)?.token().to_owned();

//     let claim = decode_jwt(token)
//     .map_err(|err| AppError::UserDoesNotExist )?.claims;

//     let db = req.extensions().get::State(&pool)()
//     .ok_or(AppError::InternalServerError)?;

//     let identity = entity::user::Entity::find()
//     .filter(entity::user::Column::Email.eq(claim.email.to_lowercase()))
//     .one(db)
//     .await.map_err(|err|  AppError::TokenCreation)?
//     .ok_or(AppError::WrongCredential)?;

//     req.extensions_mut().insert(identity);

//     Ok(next.run(req).await)
// } 