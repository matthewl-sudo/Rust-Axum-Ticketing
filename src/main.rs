#![allow(unused)]
mod utils;
mod handler;
mod model;
mod route;
mod schema;
mod error;
pub mod config;
use route::create_router;
use std::sync::Arc;
use dotenv::dotenv;
use config::Config;

use axum::{
    routing::{get, post},
    http::{StatusCode,
        header::{ACCEPT,AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method},
    extract::{Path, Query},
    Json, Router,
    response::{IntoResponse,Html}
    };
use tower_http::cors::CorsLayer;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
pub struct AppState {
    db: MySqlPool,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init();
    let database_url = std::env::var("DATABASE_URL").expect("env variable `DATABASE_URL` must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let origin_url = std::env::var("ALLOW_ORIGIN").expect("env variable `ALLOW_ORIGIN` must be set");
    let cors = CorsLayer::new()
        .allow_origin(origin_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {db:pool.clone(), env: config.clone(), })).layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}