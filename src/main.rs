use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use dcron::{
    commands::{self, AddRequest, ListResponse},
    daemon, CronEntryWithNext,
};
use std::error::Error;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let result = tokio::try_join!(daemon::watch(), run_http_server());

    if let Err(err) = result {
        error!("{err}");
    }

    info!("Terminating...");

    Ok(())
}

async fn run_http_server() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/up", get(|| async { "OK" }))
        .route("/list", get(list_action))
        .route("/add", post(add_action))
        .route("/{key}", delete(delete_action));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;

    info!("Starting server...");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn list_action() -> Json<ListResponse> {
    info!("Request: list");
    let jobs = commands::list().await.unwrap();
    Json(jobs)
}

async fn add_action(Json(body): Json<AddRequest>) -> Json<CronEntryWithNext> {
    info!("Request: add, payload = {}", body.expression);

    let response = commands::add(&body.expression).await.unwrap();
    Json(response)
}

async fn delete_action(Path(key): Path<String>) -> StatusCode {
    info!("Request: delete, key = {}", key);

    commands::delete(&key).await.unwrap();
    StatusCode::NO_CONTENT
}
