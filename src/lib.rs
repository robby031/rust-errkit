pub mod error;
pub mod kind;
pub mod mapper;
pub mod result;

pub use error::AppError;
pub use kind::ErrorKind;
pub use mapper::map_error;
pub use result::AppResult;
