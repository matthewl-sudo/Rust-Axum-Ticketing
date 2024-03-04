use axum::http::StatusCode;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, TokenData, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::utils;
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub exp: usize, // (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Issued at (as UTC timestamp)
    pub email: String,
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode>{
    dotenv().ok();
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = Claims{iat:now.timestamp() as usize, exp: (now+expire).timestamp() as usize, email:email};
    let secret: String = (env::var("TOKEN").unwrap()).clone();
    return encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|err| {
            StatusCode::INTERNAL_SERVER_ERROR
        });
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode>{
    dotenv().ok();
    let secret: String = (env::var("TOKEN").unwrap()).clone();
    let res: Result<TokenData<Claims>, StatusCode> = decode(&jwt,&DecodingKey::from_secret(secret.as_ref()),&Validation::default())
    .map_err(|_| { StatusCode::INTERNAL_SERVER_ERROR });
    return res;
}