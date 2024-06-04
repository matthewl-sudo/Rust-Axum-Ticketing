use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};
use crate::{
    handlers::{
        auth_handlers::{
         login_handler, logout_handler, register_handler, get_me_handler, refresh_token_handler}, 
        ticket_handlers::{ticket_list_handler, create_ticket_handler, delete_ticket_handler, 
        edit_ticket_handler, get_ticket_handler, health_checker_handler,}
    },
    utils::guard::auth_guard,
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/users/me",get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth_guard)),
        )
        .route("/api/refresh", post(refresh_token_handler))
        .route("/api/logout", get(logout_handler))
        .route("/api/register", post(register_handler))
        .route("/api/login", post(login_handler))
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/ticket/all", get(ticket_list_handler))
        .route("/api/ticket/", post(create_ticket_handler))
        .route("/api/ticket/:id", get(get_ticket_handler)
            .patch(edit_ticket_handler)
            .delete(delete_ticket_handler)
        )
        .with_state(app_state)
}
