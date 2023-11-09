pub mod configs;
pub mod env;
pub mod errors;

pub type AppResult<T> = std::result::Result<T, dyn std::error::Error>;
