use crate::kind::ErrorKind;
use serde::Serialize;
use std::fmt;

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

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        if let Some(ctx) = &self.context {
            write!(f, " | Source: {} | Details: {:?}", ctx.source, ctx.details)?;
        }
        Ok(())
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.kind)
    }
}

impl From<ErrorKind> for AppError {
    fn from(value: ErrorKind) -> Self {
        AppError::new(value)
    }
}
