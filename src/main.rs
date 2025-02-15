use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::{delete, get, post}, Json, Router
};
use dcron::commands::{self, AddRequest, AddResponse, ListResponse};
use std::error::Error;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/up", get(|| async { "OK" }))
        .route("/list", get(list_action))
        .route("/add", post(add_action))
        .route("/{key}", delete(delete_action));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Starting server...");
    axum::serve(listener, app).await?;
    info!("Terminating...");

    Ok(())
}

async fn list_action() -> Json<ListResponse> {
    info!("Request: list");
    let jobs = commands::list().await.unwrap();
    Json(jobs)
}

async fn add_action(Json(body): Json<AddRequest>) -> Json<AddResponse> {
    info!("Request: add, payload = {}", body.expression);

    let response = commands::add(&body.expression).await.unwrap();
    Json(response)
}

async fn delete_action(Path(key) : Path<String>) -> StatusCode {
    info!("Request: delete, key = {}", key);

    commands::delete(&key).await.unwrap();
    StatusCode::NO_CONTENT
}
