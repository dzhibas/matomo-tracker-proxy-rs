use crate::config::Config;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, response::IntoResponse};
use controllers::health;
use dotenv::dotenv;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod config;
mod controllers;
mod queue;

#[derive(Clone)]
struct AppState {
    config: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::from_env();
    let state = AppState { config };
    let shared_app_state = Arc::new(state);

    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| "matomo_tracker_proxy_rs=debug,tower_http=debug,axum::rejection=trace".into(),
        )))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .with_state(shared_app_state.clone())
        .nest("/status", health::route())
        .nest_service("/public", ServeDir::new("public"));

    let addr = "0.0.0.0:3000";

    println!("Matomo tracker proxy listening on http://{}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    format!(
        "Matomo url from config: {}",
        state
            .config
            .matomo_url
            .clone()
            .unwrap_or("Not set".to_string())
    )
}
