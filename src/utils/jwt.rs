use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, errors::Error, TokenData, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::{model::TokenClaims, utils};
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub fn token_encode(
    header: &Header,
    id: String,
    key: &EncodingKey,
) -> String {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: id,
        exp,
        iat,
    };
    let token = encode(header, &claims, key);
    return token.expect("returns encoded token string");
}

pub fn token_decode(token: String, key: &DecodingKey) 
->  Result<TokenClaims, (StatusCode, Json<ErrorResponse>)> {
    let claims = decode::<TokenClaims>(
        &token,
        key,
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: "Error",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?.claims;
    return Ok(claims);
}