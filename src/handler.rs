use std::sync::Arc;
use chrono::prelude::*;
use axum::{
    extract::{Path, Query, State},
    http::{StatusCode,header, Response}, 
    response::IntoResponse, 
    Extension, Json
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use bcrypt::{DEFAULT_COST, hash, verify};

use serde_json::{json, Value};
use sqlx::MySqlPool;
use crate::{
    error::AppError,
    model::{FilteredUser, TicketModel, TicketModelResponse, LoginModel, RegisterModel, TokenClaims},
    schema::{CreateTicketSchema, FilterOptions, UpdateTicketSchema, RegisterSchema, LoginSchema},
    AppState,
};

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

// Login Handlers -------------------------------------------
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

    let create_user = sqlx::query("INSERT INTO login (email, password, name) VALUES (?, ?, ?)")
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

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: result.id.clone().to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

// pub async fn register_handler(
//     State(data): State<Arc<AppState>>,
//     Json(req): Json<RegisterSchema>,    
// )->Result<Json<Value>, AppError>{
//     if req.email.is_empty() || req.password.is_empty() {
//         return Err(AppError::MissingCredential);
//     }
//     let result = sqlx::query_as::<_, RegisterModel>(
//         r#"SELECT email, password FROM login WHERE email = ?"#,
//     )
//     .bind(&req.email)
//     .fetch_optional(&data.db)
//     .await
//     .map_err(|e| {
//         AppError::UserAlreadyExits
//     })?;
//     if let Some(_) = result{
//         return Err(AppError::UserAlreadyExits);
//     }
//     let create_user = sqlx::query("INSERT INTO login (email, password, name) VALUES (?, ?, ?)")
//         .bind(req.email.to_string())
//         .bind(req.password.to_string())
//         .bind(req.name.to_string())
//         .execute(&data.db)
//         .await
//         .map_err(|_| AppError::InternalServerError)?;
//     if create_user.rows_affected() < 1 {
//         Err(AppError::InternalServerError)
//     } else {
//         Ok(Json(json!({ "status": "success", "result": "User successfully registered" })))
//     }
// }

// pub async fn login_handler(
//     State(data): State<Arc<AppState>>,
//     Json(req): Json<LoginSchema>,
// ) -> Result<Json<Value>, AppError> {
//     if req.email.is_empty() || req.password.is_empty() {
//         return Err(AppError::MissingCredential);
//     }
//     let result = sqlx::query_as!(LoginModel,
//         r#"SELECT * FROM login WHERE email = ?"#,
//         req.email.to_ascii_lowercase()
//         )
//         .fetch_optional(&data.db)
//         .await 
//         .map_err(| e| {
//             AppError::InternalServerError
//         }
//     );

//     if let Ok(Some( result)) = result {
//         if result.password.clone().unwrap() == req.password{
//             // println!("success {:?}", result.password.clone().unwrap());
//             let response = serde_json::json!({"status": "success", "result": serde_json::json!(result)});
//             Ok(Json(response)) 
//         } else {
//             let response = serde_json::json!({"status": "error", "result": "User password doesn't match our records"});
//             Ok(Json(response)) 
//         }
//     }
//     else{
//         Err(AppError::UserDoesNotExist)
//     }
// }

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
        r#"SELECT * FROM ticket ORDER by id LIMIT ? OFFSET ?"#,
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

    let json_response = serde_json::json!({
        "status": "success",
        "results": ticket_responses.len(),
        "tickets": ticket_responses
    });

Ok(Json(json_response))
}

pub async fn create_ticket_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTicketSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result =
        sqlx::query(r#"INSERT INTO ticket (summary, priority, status) VALUES (?, ?, ?)"#)
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
        r#"SELECT * FROM ticket WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(ticket) => {
            let ticket_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "ticket": filter_db_record(&ticket)
            })});

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
        r#"SELECT * FROM ticket WHERE id = ?"#,
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
        r#"UPDATE ticket SET summary = ?, status = ?, priority = ? WHERE id = ?"#,
    )
    .bind(body.summary.to_owned().unwrap_or_else(|| ticket.summary.clone()))
    .bind(
        body.status
            .to_owned()
            .unwrap_or_else(|| ticket.status.clone()))
    .bind(body.priority
        .to_owned()
        .unwrap_or_else(|| ticket.priority.clone()))
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
        r#"SELECT * FROM ticket WHERE id = ?"#,
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

    let ticket_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "ticket": filter_db_record(&updated_ticket)
    })});

    Ok(Json(ticket_response))
}

pub async fn delete_ticket_handler(
    Path(id): Path<i64>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query!(r#"DELETE FROM ticket WHERE id = ?"#, id.to_string())
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

fn filter_user_record(result: &LoginModel) -> FilteredUser {
    FilteredUser {
        id: result.id.to_string(),
        email: result.email.clone().unwrap(),
        name: result.name.clone().unwrap(),
        role: result.role.clone().unwrap(),
    }
}