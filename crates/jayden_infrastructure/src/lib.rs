use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn initialized_database(dsn: &str, max_conns: u32) -> Pool<Postgres> {
  let database = PgPoolOptions::new().max_connections(max_conns).connect(dsn).await.unwrap();

  sqlx::migrate!().run(&database).await.expect("Connect run migrations");
  database
}
