use axum::{routing, Router};
use dotenv::dotenv;
use jayden_core::{cfgs::Config, context::AppContext};
use jayden_infrastructure::initialized_database;
use tracing::info;

#[tokio::main]
async fn main() {
  // Region:     --- Tracing Initial
  let subscriber = tracing_subscriber::fmt::Subscriber::builder().with_max_level(tracing::Level::INFO).finish();
  tracing::subscriber::set_global_default(subscriber).expect("Cannot setting subscriber global");
  // End-Region: --- Tracing Initial

  // Region:     --- ENV Setup
  info!("Loading Environment & Migrating Database!");
  dotenv().ok();
  let config = Config::from_env().map_err(|e| tracing::error!("Loading environment Error: {}", e.to_string())).unwrap();
  // End-Region: --- ENV Setup

  // Region:     --- database
  let pool = initialized_database(&config.postgres.dsn, config.postgres.max_conns).await;
  let context = AppContext::new(pool);
  info!("Migrations successfully ran! Initializing axum server... ");
  // End-Region: --- database

  let app = Router::new().route("/", routing::get(hello_world)).with_state(context);
  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.web.addr)).await.unwrap();

  info!("Server is running on port: {}", config.web.addr);
  axum::serve(listener, app).await.unwrap()
}

async fn hello_world() -> &'static str {
  "Hello, World!"
}
