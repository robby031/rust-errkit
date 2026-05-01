use rust_errkit::{
    AppError,
    kind::{ErrorKind, ErrorReason},
};

fn main() {
    let err = AppError::from(ErrorKind::network(ErrorReason::Timeout));
    println!("MESSAGE: {}", err.kind.message());
}
