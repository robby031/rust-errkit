pub mod error;
pub mod kind;
pub mod mapper;

pub use error::AppError;
pub use kind::ErrorKind;
pub use mapper::{from_any, map_error};
pub use result::AppResult;
