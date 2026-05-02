use rust_errkit::{
    AppError,
    kind::{ErrorKind, ErrorReason},
};

fn main() {
    let err: AppError = ErrorKind::network(ErrorReason::Timeout).into();
    println!("MESSAGE: {}", err);
}
