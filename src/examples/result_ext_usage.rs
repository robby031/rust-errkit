use rust_errkit::{AppError, AppResult, ResultExt, kind::ErrorKind};

fn might_fail(success: bool) -> Result<String, &'static str> {
    if success {
        Ok("OK".to_string())
    } else {
        Err("something went wrong")
    }
}

fn main() -> AppResult<()> {
    let value = might_fail(false).map_err(|e| {
        AppError::from(ErrorKind::unknown()).with_context("might_fail", Some(e.to_string()))
    })?;

    println!("{}", value);
    Ok(())
}
