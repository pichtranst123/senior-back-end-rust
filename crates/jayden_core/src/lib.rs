use errs::AppError;

pub mod cfgs;
pub mod context;
pub mod errs;
pub mod resps;
pub mod utils;

pub type AppResult<T> = std::result::Result<T, AppError>;
