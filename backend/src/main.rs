use axum::{Router, routing::get};

const PORT: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(async || "Hello, world!"));

    let listener = tokio::net::TcpListener::bind(PORT).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
