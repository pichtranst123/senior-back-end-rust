use serde::{Deserialize, Serialize};

use super::UserDto;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct UserAuthResponse {
  pub user: UserDto,
}

impl UserAuthResponse {
  pub fn default_stub(
    id: i64,
    username: String,
    email: Option<String>,
    public_key: Option<String>,
    bio: Option<String>,
    password: Option<String>,
    image: Option<String>,
  ) -> Self {
    Self { user: UserDto { id, username, email, bio, image, password, public_key } }
  }
}
