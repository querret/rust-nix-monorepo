use std::collections::HashMap;

use axum::{
    routing::get,
    Json, Router,
};
use common::{Message, RepoInfo};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct GithubRepo {
    name: String,
    description: Option<String>,
    #[serde(rename = "language")]
    primary_language: Option<String>,
}

async fn health() -> Json<Message> {
    Json(Message::new("Web service healthy".into()))
}

async fn repo() -> Json<RepoInfo> {
    let client = reqwest::Client::new();

    let repo_fut = client
        .get("https://api.github.com/repos/querret/rust-nix-monorepo")
        .header("User-Agent", "rust-nix-monorepo-demo")
        .send();

    let langs_fut = client
        .get("https://api.github.com/repos/querret/rust-nix-monorepo/languages")
        .header("User-Agent", "rust-nix-monorepo-demo")
        .send();

    let (repo_res, langs_res) = tokio::join!(repo_fut, langs_fut);

    let repo: GithubRepo = repo_res.unwrap().json().await.unwrap();
    let langs_map: HashMap<String, u64> = langs_res.unwrap().json().await.unwrap();

    let mut languages: Vec<(String, u64)> = langs_map.into_iter().collect();
    languages.sort_by(|a, b| b.1.cmp(&a.1));

    let languages: Vec<String> = languages.into_iter().map(|(lang, _)| lang).collect();


    Json(RepoInfo {
        name: repo.name,
        description: repo.description,
        primary_language: repo.primary_language,
        languages,
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
        .nest_service("/", ServeDir::new("static"));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Web service listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}