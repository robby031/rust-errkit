use rust_errkit::result::ResultExt;
use rust_errkit::{
    AppError, AppResult,
    kind::{ErrorDomain, ErrorKind, ErrorReason},
};

#[test]
fn test_error_reason_message() {
    let reason = ErrorReason::Timeout;

    assert_eq!(reason.message(), "Request timeout.");
}

#[test]
fn test_error_kind_creation() {
    let kind = ErrorKind::network(ErrorReason::Timeout);

    match kind.domain {
        ErrorDomain::Network => {}
        _ => panic!("wrong domain"),
    }

    assert_eq!(kind.message(), "Request timeout.");
}

#[test]
fn test_app_error_creation() {
    let err = AppError::from(ErrorKind::db(ErrorReason::ConnectionFailed));

    assert_eq!(err.kind.message(), "Connection failed.");
    assert!(err.context.is_none());
}

#[test]
fn test_app_error_context() {
    let err = AppError::from(ErrorKind::network(ErrorReason::Timeout))
        .with_context("reqwest", Some("timeout at 10s".to_string()));

    let ctx = err.context.unwrap();

    assert_eq!(ctx.source, "reqwest");
    assert_eq!(ctx.details.unwrap(), "timeout at 10s");
}

#[test]
fn test_unknown_error() {
    let err = AppError::unknown();

    assert_eq!(err.kind.message(), "Unexpected error occurred.");
}

#[test]
fn test_result_ext_ok() {
    let res: AppResult<i32> = Ok(10);

    let out = res.errkit();

    assert!(out.is_ok());
    assert_eq!(out.unwrap(), 10);
}

#[test]
fn test_result_ext_err() {
    let res: AppResult<i32> = Err(AppError::unknown());

    let out = res.errkit();

    assert!(out.is_err());
}

#[test]
fn test_full_error_flow() {
    let err = AppError::from(ErrorKind::auth(ErrorReason::Unauthorized))
        .with_context("auth_service", None);

    assert_eq!(err.kind.message(), "Unauthorized access.");
    let ctx = err.context.unwrap();
    assert_eq!(ctx.source, "auth_service");
}
