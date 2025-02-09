use axum::{routing::{get, post}, Json, Router};
use tracing::info;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
enum DcronJob {
    Upload(String),
    List,
}

#[derive(Serialize, Deserialize, Debug)]
struct JobList(Vec<String>);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/up", get(|| async { "OK" }))
        .route("/list", get(list))
        .route("/upload", post(upload));
    
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        info!("Starting server...");
        axum::serve(listener, app).await.unwrap();
}


async fn list() -> Json<JobList> {
    info!("Request: List Jobs");
    Json(JobList(vec!["Job1".into(), "Job2".into()]))
}

async fn upload(Json(body): Json<JobList>) -> Json<String> {
    info!("Request: Upload, jobs = {:?}", body);
    Json("OK".into())
}
