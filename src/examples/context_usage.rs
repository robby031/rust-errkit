use rust_errkit::{
    AppError,
    kind::{ErrorKind, ErrorReason},
};

fn main() {
    let err = AppError::from(ErrorKind::db(ErrorReason::ConnectionFailed))
        .with_context("sqlx", Some("connection refused".to_string()));

    println!("MESSAGE: {}", err.kind.message());

    if let Some(ctx) = err.context {
        println!("SOURCE: {}", ctx.source);
        println!("DETAILS: {:?}", ctx.details);
    }
}
