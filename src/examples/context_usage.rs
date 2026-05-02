use rust_errkit::{
    AppError,
    kind::{ErrorKind, ErrorReason},
};

fn main() {
    let err = AppError::from(ErrorKind::db(ErrorReason::ConnectionFailed)).with_context(
        "sqlx_pool",
        Some("Connection refused by database peer".to_string()),
    );

    println!("Detailed Error:\n{}", err);
}
