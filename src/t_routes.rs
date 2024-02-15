use axum::{Router,routing::{get, post}};
use crate::*;


pub fn test_routes() -> Router{
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/groot", get(groot))
        .route("/hello/:name", get(fello))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))

}