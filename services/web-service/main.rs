use axum::{routing::get, Json, Router};
use common::{Message, RepoInfo};
use serde::Deserialize;

#[derive(Deserialize)]
struct CodebergRepo {
    name: String,
    description: Option<String>,
    stars_count: u32,
    language: Option<String>,
}

async fn root() -> Json<Message> {
    Json(Message::new("Hello from web-service".into()))
}
