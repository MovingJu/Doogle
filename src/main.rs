use axum::{Router, routing::get, routing::post};

async fn post_documents() -> &'static str {
    todo!("document indexing endpoint — see issue #12")
}

async fn get_search() -> &'static str {
    todo!("search endpoint — see issue #12")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/documents", post(post_documents))
        .route("/search", get(get_search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
