use axum::{routing, Router};
use tracing::info;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = Router::new().route("/", routing::get(hello_world));

  let addr = "0.0.0.0:8888";
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

  info!("Server is running on port: {}", addr);
  axum::serve(listener, app).await.unwrap()
}

async fn hello_world() -> &'static str {
  "Hello, World!"
}
