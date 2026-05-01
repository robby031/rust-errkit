use crate::AppError;

pub type AppResult<T> = Result<T, AppError>;

pub trait ResultExt<T> {
    fn errkit(self) -> AppResult<T>;
}

impl<T> ResultExt<T> for Result<T, AppError> {
    fn errkit(self) -> AppResult<T> {
        self
    }
}
