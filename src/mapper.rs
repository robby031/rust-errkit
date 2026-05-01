use crate::{
    AppError,
    kind::{ErrorDomain, ErrorKind, ErrorReason},
};

pub fn map_error(domain: ErrorDomain, reason: ErrorReason) -> AppError {
    AppError::from(ErrorKind::new(domain, reason))
}
