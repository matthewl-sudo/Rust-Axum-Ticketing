use std::sync::Arc;
use chrono::prelude::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    body::Body,
    response::IntoResponse, 
    Extension, Json
};

use serde_json::{json, Value};
use sqlx::MySqlPool;
use crate::{
    error::AppError, 
    model::{CommentModel, CommentModelResponse, LoginModel}, 
    schema::{CreateCommentSchema, FilterOptions}, 
    AppState
};

pub async fn create_comment_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateCommentSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result =
        sqlx::query(r#"INSERT INTO comments (content, ticket_id, author_id) VALUES (?, ?, ?)"#)
            .bind(body.content.to_string())
            .bind(body.ticket_id.to_owned())
            .bind(body.author_id.to_owned())
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Ticket with that title already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }      
    let response_status = serde_json::json!({"status": "success"});
    Ok(Json(response_status))
    

}

pub async fn comments_list_handler(
    Path(ticket_id): Path<i64>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

        let query_result = sqlx::query_as!(
        CommentModel,
        r#"SELECT c.id, 
        c.content, c.create_date, 
        c.author_id, login.name 
        FROM comments c JOIN login 
        ON c.author_id = login.id WHERE c.ticket_id = ?"#,
        ticket_id.to_string()
    )
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let comment_responses = query_result
    .iter()
    .map(|comment| filter_comment_record(&comment))
    .collect::<Vec<CommentModelResponse>>();

    let json_response = serde_json::json!(comment_responses);
    //     "status": "success",
    //     "results": comment_responses.len(),
    //     "comments": comment_responses
    // });

Ok(Json(json_response))
}

fn filter_comment_record(comment: &CommentModel) -> CommentModelResponse {
    CommentModelResponse {
        id: comment.id.to_owned(),
        content: comment.content.to_owned(),
        author_id: comment.author_id.to_owned(),
        create_date: comment.create_date,
        name: comment.name.to_owned().expect("msg"),
    }
}