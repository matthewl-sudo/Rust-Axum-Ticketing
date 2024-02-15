#![allow(unused)]
#[warn(dead_code)]
mod t_routes;
mod error;
pub use self::error::{Error, Result}; 

use axum::{
    routing::{get, post},
    http::StatusCode,
    extract::{Path, Query},
    Json, Router,
    response::{IntoResponse,Html}
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new().merge(t_routes::test_routes());


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
async fn groot() -> impl IntoResponse {
   Html("<h1>Hello, Groot!</h1>")
}
async fn hello(Query(params):Query<HelloParams>) -> impl IntoResponse{
    let name = params.name.as_deref().unwrap_or("stranger");
    Html(format!("<h1>Hello <strong>{name}</strong></h1>"))
} 
async fn fello(Path(name): Path<String>) -> impl IntoResponse{
    Html(format!("<p>Hello <strong>{name}</strong></p>"))
} 

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}
struct HelloParams{
    name: Option<String>,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}