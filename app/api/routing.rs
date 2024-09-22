use axum::{
    http::StatusCode,
    routing::{any, get, post},
    Router,
};

pub mod auth;
use auth::route_handler;

pub fn api_routes() -> Router {
    return Router::new()
        .route("/auth/signin", get(route_handler))
        .route("/auth/signin", post(route_handler))
        .route("/health-check", any(|| (async { StatusCode::NOT_FOUND })))
        .fallback(|| (async { StatusCode::NOT_FOUND }));
}
