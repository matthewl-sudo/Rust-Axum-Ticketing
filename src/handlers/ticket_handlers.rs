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
    model::{TicketModel, TicketModelResponse}, 
    schema::{CreateTicketSchema, FilterOptions, UpdateTicketSchema}, 
    AppState
};


// Ticket Handlers ------------------------------------------
pub async fn ticket_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();
    
    let limit = opts.limit.unwrap_or(20);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    
    let tickets = sqlx::query_as!(
        TicketModel,
        r#"SELECT * FROM tickets ORDER by create_date DESC LIMIT ? OFFSET ?"#,
        limit as i64,
        offset as i64
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
    
    let ticket_responses = tickets
    .iter()
    .map(|ticket| filter_db_record(&ticket))
    .collect::<Vec<TicketModelResponse>>();

    let json_response = serde_json::json!(ticket_responses);
    //     "status": "success",
    //     "results": ticket_responses.len(),
    //     "tickets": ticket_responses
    // });

Ok(Json(json_response))
}

pub async fn create_ticket_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTicketSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result =
        sqlx::query(r#"INSERT INTO tickets (summary, priority, status) VALUES (?, ?, ?)"#)
            .bind(body.summary.to_string())
            .bind(body.priority.to_string())
            .bind(body.status.to_string())
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

pub async fn get_ticket_handler(
    Path(id): Path<i64>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        TicketModel,
        r#"SELECT * FROM tickets WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;


    match query_result {
        Ok(ticket) => {
            let ticket_response = serde_json::json!(filter_db_record(&ticket));

            return Ok(Json(ticket_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Ticket with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    };
}

pub async fn edit_ticket_handler(
    Path(id): Path<i64>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateTicketSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        TicketModel,
        r#"SELECT * FROM tickets WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    let ticket = match query_result {
        Ok(ticket) => ticket,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Ticket with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    };

    let update_result = sqlx::query(
        r#"UPDATE tickets SET summary = ?, status = ?, priority = ?, update_date = ? WHERE id = ?"#,
    )
    .bind(body.summary.to_owned().unwrap_or_else(|| ticket.summary.clone()))
    .bind(
        body.status
            .to_owned()
            .unwrap_or_else(|| ticket.status.clone()))
    .bind(body.priority
        .to_owned()
        .unwrap_or_else(|| ticket.priority.clone()))
    .bind(chrono::offset::Utc::now())
    .bind(id.to_string())
    .execute(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Ticket with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let updated_ticket = sqlx::query_as!(
        TicketModel,
        r#"SELECT * FROM tickets WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    let ticket_response = serde_json::json!({
        "ticket": filter_db_record(&updated_ticket),
        "status": "success",
    });

    Ok(Json(ticket_response))
}

pub async fn delete_ticket_handler(
    Path(id): Path<i64>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query!(r#"DELETE FROM tickets WHERE id = ?"#, id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;

    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Ticket with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}


fn filter_db_record(ticket: &TicketModel) -> TicketModelResponse {
    TicketModelResponse {
        id: ticket.id.to_owned(),
        // title: ticket.title.to_owned(),
        summary: ticket.summary.to_owned(),
        status: ticket.status.to_owned(),
        priority: ticket.priority.to_owned(),
        createdAt: ticket.create_date.unwrap(),
        updatedAt: ticket.update_date.unwrap(),
    }
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Rust CRUD API Example with Axum Framework and MySQL";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
