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

async fn repo() -> Result<Json<RepoInfo>, String> {
    let url = "https://codeberg.org/api/v1/repos/querret/rust-nix-monorepo";
    
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "rust-nix-monorepo-demo")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let repo: CodebergRepo = response
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}",e))?;
    
    Ok(Json(RepoInfo {
        name: repo.name,
        description: repo.description,
        stars: repo.stars_count,
        language: repo.language,
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/repo", get(repo));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Web service listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
