use serde::{Deserialize, Serialize};

pub mod requests;
pub mod responses;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserDto {
  pub id: i64,
  pub username: String,
  pub email: Option<String>,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub password: Option<String>,
  pub public_key: Option<String>,
}
