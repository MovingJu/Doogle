use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;

use doogle::{Document, Index, SearchResult};

type SharedIndex = Arc<Mutex<Index>>;

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    10
}

async fn index_documents(
    State(index): State<SharedIndex>,
    Json(docs): Json<Vec<Document>>,
) -> Json<serde_json::Value> {
    let mut idx = index.lock().unwrap();
    let count = docs.len();
    for doc in docs {
        idx.add_document(doc);
    }
    Json(serde_json::json!({ "indexed": count, "total_documents": idx.document_count() }))
}

async fn search(
    State(index): State<SharedIndex>,
    Query(params): Query<SearchParams>,
) -> Json<Vec<SearchResult>> {
    let idx = index.lock().unwrap();
    Json(idx.search(&params.q, params.limit))
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let index: SharedIndex = Arc::new(Mutex::new(Index::new()));

    let app = Router::new()
        .route("/health", get(health))
        .route("/documents", post(index_documents))
        .route("/search", get(search))
        .layer(CorsLayer::permissive())
        .with_state(index);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8420").await.unwrap();
    println!("🔎 doogle listening on http://0.0.0.0:8420");
    axum::serve(listener, app).await.unwrap();
}
