use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

pub fn route() -> Router {
    Router::new().route("/health", get(health))
}

pub async fn health() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}
