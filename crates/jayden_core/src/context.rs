use std::sync::Arc;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppContext {
  pub pool: Pool<Postgres>,
}

impl AppContext {
  pub fn new(pool: Pool<Postgres>) -> Arc<AppContext> {
    Arc::new(Self { pool })
  }
}
