use serde::Serialize;

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
    Timeout,
    NotFound,
    Unauthorized,
    Forbidden,
    InvalidInput,
    ConnectionFailed,
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

impl ErrorReason {
    pub fn message(&self) -> &'static str {
        match self {
            Self::Timeout => "Request timeout.",
            Self::NotFound => "Data not found.",
            Self::Unauthorized => "Unauthorized access.",
            Self::Forbidden => "Access denied.",
            Self::InvalidInput => "Invalid input.",
            Self::ConnectionFailed => "Connection failed.",
            Self::Unexpected => "Unexpected error occurred.",
        }
    }
}
