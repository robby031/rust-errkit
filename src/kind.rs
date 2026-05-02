use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub enum ErrorDomain {
    Core,
    Network,
    Database,
    IO,
    Auth,
    External,
}

#[derive(Debug, Clone, Serialize)]
pub enum ErrorReason {
    // Infrastructure
    Timeout,
    ConnectionFailed,
    ServerOverload,

    // Data Resource
    NotFound,
    AlreadyExists,
    ResourceExhausted,

    // Auth
    Unauthorized,
    Forbidden,
    SessionExpired,
    InvalidToken,

    // Business
    InvalidInput,
    ConstraintViolation,
    InconsistentState,
    ProcessingFailed,

    // Security
    ChecksumMismatch,
    DataCorruption,

    // More
    Unexpected,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorKind {
    pub domain: ErrorDomain,
    pub reason: ErrorReason,
}

impl ErrorKind {
    pub fn message(&self) -> &'static str {
        self.reason.message()
    }

    pub fn new(domain: ErrorDomain, reason: ErrorReason) -> Self {
        Self { domain, reason }
    }

    pub fn core(reason: ErrorReason) -> Self {
        Self::new(ErrorDomain::Core, reason)
    }

    pub fn network(reason: ErrorReason) -> Self {
        Self::new(ErrorDomain::Network, reason)
    }

    pub fn auth(reason: ErrorReason) -> Self {
        Self::new(ErrorDomain::Auth, reason)
    }

    pub fn db(reason: ErrorReason) -> Self {
        Self::new(ErrorDomain::Database, reason)
    }

    pub fn io(reason: ErrorReason) -> Self {
        Self::new(ErrorDomain::IO, reason)
    }

    pub fn unknown() -> Self {
        Self::new(ErrorDomain::External, ErrorReason::Unexpected)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?}] {:?}: {}",
            self.domain,
            self.reason,
            self.message()
        )
    }
}

impl std::error::Error for ErrorKind {}

impl ErrorReason {
    pub fn message(&self) -> &'static str {
        match self {
            Self::Timeout => "The operation took too long to respond. Please try again later.",
            Self::ConnectionFailed => "Unable to establish a connection to the server or service.",
            Self::ServerOverload => "Server is currently under heavy load. Scaling in progress...",

            Self::Unauthorized => "Authentication is required to access this resource.",
            Self::Forbidden => "You do not have the necessary permissions for this action.",
            Self::SessionExpired => "Your session has timed out. Please log in again.",
            Self::InvalidToken => "The provided security token is invalid or malformed.",

            Self::NotFound => "The requested resource could not be found in our records.",
            Self::AlreadyExists => {
                "Resource conflict: An entry with this identifier already exists."
            }
            Self::ResourceExhausted => "Daily limit exceeded or system resources are depleted.",

            Self::InvalidInput => "The provided data is invalid or does not meet the requirements.",
            Self::ConstraintViolation => {
                "Operation rejected: Business rule or constraint violation."
            }
            Self::InconsistentState => "Internal data mismatch detected. Please refresh and retry.",
            Self::ProcessingFailed => {
                "The request was valid but we encountered an error during execution."
            }

            Self::ChecksumMismatch => "Integrity check failed. Data may have been tampered with.",
            Self::DataCorruption => "Internal data structures appear to be corrupted.",

            Self::Unexpected => {
                "An unhandled exception occurred. Our engineers have been notified."
            }
        }
    }
}
