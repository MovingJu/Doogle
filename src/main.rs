use axum::{Router, routing::get, routing::post};

async fn post_documents() -> &'static str {
    todo!("문서 색인 엔드포인트 — 이슈 #12 참고")
}

async fn get_search() -> &'static str {
    todo!("검색 엔드포인트 — 이슈 #12 참고")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/documents", post(post_documents))
        .route("/search", get(get_search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
