use serde::{Deserialize, Serialize};

use chrono::prelude::*;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize, // (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Issued at (as UTC timestamp)
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct LoginModel {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,

}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct RegisterModel {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

// the input to our handler
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct TicketModel {
    pub id: i64,
    // pub title: String,
    pub summary: String,
    pub priority: String,
    pub status: String,
    pub create_date: Option<chrono::DateTime<chrono::Utc>>,
    pub update_date: Option<chrono::DateTime<chrono::Utc>>,
}

// the output to our handler
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TicketModelResponse {
    pub id: i64,
    // pub title: String,
    pub summary: String,
    pub priority: String,
    pub status: String,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
}
