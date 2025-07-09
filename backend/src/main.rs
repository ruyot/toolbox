mod endpoints;

use axum::{
    Router,
    routing::{get, post},
};
use tracing::Level;

const PORT: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let t_sub = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG) // initialize max log level
        .finish();
    tracing::subscriber::set_global_default(t_sub).unwrap(); // set as global trace sub

    let app = Router::new()
        .route("/", get(async || "Hello, world!"))
        .route("/register", post(endpoints::register));

    let listener = tokio::net::TcpListener::bind(PORT).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
