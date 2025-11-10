use axum::{
    routing::get,
    Json, Router,
};
use common::{Message, RepoInfo};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct CodebergRepo {
    name: String,
    description: Option<String>,
    stars_count: u32,
    language: Option<String>,
}

async fn health() -> Json<Message> {
    Json(Message::new("Web service healthy".into()))
}

async fn repo() -> Json<RepoInfo> {
    let url = "https://codeberg.org/api/v1/repos/querret/rust-nix-monorepo";
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "rust-nix-monorepo-demo")
        .send()
        .await
        .unwrap();
    
    let repo: CodebergRepo = response.json().await.unwrap();
    
    Json(RepoInfo {
        name: repo.name,
        description: repo.description,
        stars: repo.stars_count,
        language: repo.language,
    })
}

#[tokio::main]
async fn main() {
    // API routes
    let api_routes = Router::new()
        .route("/health", get(health))
        .route("/repo", get(repo));
    
    // Serve static files from ./static
    let app = Router::new()
        .nest("/api", api_routes)
        .nest_service("/", ServeDir::new("services/web-service/static"));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Web service listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}