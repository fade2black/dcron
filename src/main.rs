use axum::{routing::{get, post}, Json, Router};
use dcron::cron_utils;
use tracing::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct AddRequest {
    expression: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct AddResponse {
    pattern: String,
    next: String,
    command: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Job(String);
#[derive(Serialize, Deserialize, Debug)]
struct JobList(Vec<Job>);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/up", get(|| async { "OK" }))
        .route("/list", get(list))
        .route("/add", post(add));
    
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        info!("Starting server...");
        axum::serve(listener, app).await.unwrap();
}

async fn list() -> Json<JobList> {
    info!("Request: List Jobs");
    Json(JobList(vec![Job("Job1".into()), Job("Job2".into())]))
}

async fn add(Json(body): Json<AddRequest>) -> Json<AddResponse> {
    info!("Request: add, payload = {}", body.expression);
    let cron = cron_utils::CronEntry::new(&body.expression).unwrap();

    let response = AddResponse {
        pattern: cron.pattern(),
        next: cron.next_occurrence().unwrap().to_string(),
        command: cron.command,
    };
    Json(response)
}
