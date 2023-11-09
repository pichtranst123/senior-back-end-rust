use std::net::SocketAddr;

use axum::{routing, Router, Server};
use tracing::info;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let app = Router::new().route("/", routing::get(|| async { "Hello!" }));
  let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8888));

  info!("Server is running on port: {}", addr);
  Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
