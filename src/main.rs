use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use controllers::health;
use dotenv::dotenv;

mod controllers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(root))
        .nest("/status", health::route());

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
