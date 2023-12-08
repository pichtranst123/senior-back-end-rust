use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Default, Validate, Clone)]
pub struct RegisterUserDto {
  #[validate(required, length(min = 4))]
  pub username: Option<String>,
  #[validate(required)]
  pub publickey: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, Validate)]
pub struct RegisterUserRequest {
  #[validate]
  pub user: RegisterUserDto,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct LoginUserDto {
  #[validate(required, length(min = 4))]
  pub username: Option<String>,
  #[validate(required)]
  pub publickey: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct LoginUserRequest {
  #[validate]
  pub user: LoginUserDto,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserDto {
  pub username: Option<String>,
  pub email: Option<String>,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub publickey: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserRequest {
  pub user: UpdateUserDto,
}
