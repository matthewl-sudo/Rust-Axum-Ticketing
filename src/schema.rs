use serde::{Deserialize, Serialize};

// Structs that will be used to deserialize the request 
// parameters and bodies in the Axum route functions and also
// ensure that the required fields are included in the JSON object.

// Serialization is used when you pass data from backend to the frontend
// and Deserialization from frontend to backend.

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthStateSchema {
    #[serde(skip_serializing)]
    pub token: Option<String>,
    pub logged_in: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTicketSchema {
    // pub title: String,
    pub summary: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTicketSchema {
    // pub title: Option<String>,
    pub summary: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
}

// #[derive(Deserialize, Debug)]
// pub struct ParamOptions {
//     pub id: i64,
// }