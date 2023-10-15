use crate::config::Config;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use controllers::health;
use dotenv::dotenv;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod controllers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "matomo_tracker_proxy_rs=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new();

    let app = Router::new()
        .route("/", get(root))
        .with_state(config)
        .nest("/status", health::route())
        .nest_service("/public", ServeDir::new("public"));

    let addr = "0.0.0.0:3000";

    println!("Matomo tracker proxy listening on http://{}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    "Root"
}
