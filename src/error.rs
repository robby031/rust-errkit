use crate::kind::{ErrorDomain, ErrorKind, ErrorReason};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub kind: ErrorKind,
    pub message: String,
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
            message: kind.message().to_string(),
            kind,
            context: None,
        }
    }

    pub fn with_context(mut self, source: &'static str, details: Option<String>) -> Self {
        self.context = Some(ErrorContext { source, details });
        self
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = msg.into();
        self
    }

    pub fn unknown() -> Self {
        Self::new(ErrorKind::Unknown)
    }

    fn default_message(kind: &ErrorKind) -> String {
        match (&kind.domain, &kind.reason) {
            // NETWORK DOMAIN
            (ErrorDomain::Network, ErrorReason::Timeout) => {
                "Request timeout, try again.".to_string()
            }
            (ErrorDomain::Network, ErrorReason::ConnectionFailed) => {
                "Failed connection to server.".to_string()
            }

            // AUTH DOMAIN
            (ErrorDomain::Auth, ErrorReason::Unauthorized) => {
                "You are not logged in or your session has expired.".to_string()
            }
            (ErrorDomain::Auth, ErrorReason::Forbidden) => "Access denied.".to_string(),

            // CORE DOMAIN
            (ErrorDomain::Core, ErrorReason::InvalidInput) => "Invalid input.".to_string(),

            // FALLBACK GLOBAL
            _ => "An unknown error occurred.".to_string(),
        }
    }
}

impl From<ErrorKind> for AppError {
    fn from(value: ErrorKind) -> Self {
        AppError::new(value)
    }
}
