use rust_errkit::{
    AppError, AppResult,
    kind::{ErrorKind, ErrorReason},
};

fn might_fail(success: bool) -> Result<String, &'static str> {
    if success {
        Ok("Success!".to_string())
    } else {
        Err("Internal logic leak")
    }
}

fn main() -> AppResult<()> {
    let value = might_fail(false).map_err(|e| {
        AppError::from(ErrorKind::core(ErrorReason::Unexpected))
            .with_context("logic_engine", Some(e.to_string()))
    })?;

    println!("{}", value);
    Ok(())
}
