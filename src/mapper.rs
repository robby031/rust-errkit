use crate::{
    AppError,
    kind::{ErrorDomain, ErrorKind, ErrorReason},
};

pub trait ErrorMapper {
    fn map(&self) -> AppError;
}
