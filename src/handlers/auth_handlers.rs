use std::sync::Arc;
use chrono::prelude::*;
use axum::{
    extract::State,
    http::{header, HeaderMap, Response, Request, StatusCode},
    body::Body,
    response::IntoResponse, 
    Extension, Json
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{ EncodingKey, Header};
use bcrypt::{DEFAULT_COST, hash, verify};

use serde_json::{json, Value};
use sqlx::MySqlPool;
use crate::{
    error::AppError, 
    model::{FilteredUser, LoginModel, RegisterModel}, 
    schema::{FilterOptions, LoginSchema, RegisterSchema}, 
    utils::jwt::token_encode, AppState
};
// Auth Handlers -------------------------------------------

pub async fn refresh_token_handler(
    headers: HeaderMap,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
 ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let token = req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                });
    Ok(Json(json!({"access_token": token})))
}

pub async fn get_me_handler(
    Extension(result): Extension<LoginModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&result)
        })
    });

    Ok(Json(json_response))
}

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // clear token to logout
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "message": "successfully logged out"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    Json(req): Json<RegisterSchema>,    
)->Result<Json<Value>, AppError>{
    if req.email.is_empty() || req.password.is_empty() {
        return Err(AppError::MissingCredential);
    }
    let result = sqlx::query_as::<_, RegisterModel>(
        r#"SELECT email, password FROM login WHERE email = ?"#,
    )
    .bind(&req.email)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        AppError::UserAlreadyExits
    })?;
    if let Some(_) = result{
        return Err(AppError::UserAlreadyExits);
    }
    let hashed_password = match hash(req.password.clone().to_string(), DEFAULT_COST){
        Ok(h) => h,
        Err(e) => panic!("Error hashing {:}", e),
    };

    let create_user = sqlx::query("INSERT INTO login (name, email, password) VALUES (?, ?, ?)")
        .bind(req.name.to_string())
        .bind(req.email.to_string())
        .bind(&hashed_password)
        .execute(&data.db)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    if create_user.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({ "status": "success", "result": "User successfully registered" })))
    }
}

pub async fn login_handler(
    State(data): State<Arc<AppState>>,
    Json(req): Json<LoginSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let result = sqlx::query_as!(
        LoginModel,
        r#"SELECT * FROM login WHERE email = ?"#,
        req.email.to_ascii_lowercase()
        )
        .fetch_optional(&data.db)
        .await 
        .map_err(| e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": "Invalid email",
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;
    // compare request password with hashed password in db and return true or false
    let is_valid = match verify(
        &req.password.as_bytes(),
        &result.password.clone().unwrap()){
        Ok(true) => true,
        Ok(false) => false,
        Err(err) => false,
    };

    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let token = token_encode(&Header::default(), result.id.clone().to_string(), &EncodingKey::from_secret(data.env.jwt_secret.as_ref()));

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "token": token, "user": filter_user_record(&result)}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

fn filter_user_record(result: &LoginModel) -> FilteredUser {
    FilteredUser {
        id: result.id.to_string(),
        email: result.email.clone().unwrap(),
        name: result.name.clone().unwrap(),
        role: result.role.clone().unwrap(),
    }
}
