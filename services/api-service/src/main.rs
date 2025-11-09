use axum::{routing::get, Json, Router};
use common::Message;

async fn status() -> Json<Message> {
    Json(Message::new("API service healthy".into()))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/status", get(status));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    println!("API service listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
