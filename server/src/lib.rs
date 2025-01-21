use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
pub async fn start() {
  let app = Router::new().route("/", get(|| async { "Hello, world!" }));
  let port: u16 = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("Failed to parse PORT");
  let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
  let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
