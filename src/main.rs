use axum::{
    routing::{get, post},
    Json, Router,
};
use dcron::commands::{self, AddRequest, AddResponse};
use std::error::Error;

use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
struct Job(String);
#[derive(Serialize, Deserialize, Debug)]
struct JobList(Vec<Job>);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/up", get(|| async { "OK" }))
        .route("/list", get(list_action))
        .route("/add", post(add_action));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Starting server...");
    axum::serve(listener, app).await?;
    info!("Terminating...");

    Ok(())
}

async fn list_action() -> Json<JobList> {
    info!("Request: List Jobs");
    Json(JobList(vec![Job("Job1".into()), Job("Job2".into())]))
}

async fn add_action(Json(body): Json<AddRequest>) -> Json<AddResponse> {
    info!("Request: add, payload = {}", body.expression);

    let response = commands::add(&body.expression).unwrap();
    Json(response)
}
