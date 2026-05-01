use crate::kind::ErrorKind;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub kind: ErrorKind,
    pub context: Option<ErrorContext>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorContext {
    pub source: &'static str,
    pub details: Option<String>,
}

impl AppError {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            context: None,
        }
    }

    pub fn with_context(mut self, source: &'static str, details: Option<String>) -> Self {
        self.context = Some(ErrorContext { source, details });
        self
    }

    pub fn unknown() -> Self {
        Self::from(ErrorKind::unknown())
    }
}

impl From<ErrorKind> for AppError {
    fn from(value: ErrorKind) -> Self {
        AppError::new(value)
    }
}
