use rust_errkit::{
    AppError, AppResult,
    kind::{ErrorKind, ErrorReason},
};

async fn fetch_remote() -> Result<String, &'static str> {
    Err("timeout")
}

fn map_remote_error(_e: &'static str) -> AppError {
    AppError::from(ErrorKind::network(ErrorReason::Timeout))
}

async fn service_layer() -> AppResult<String> {
    let data = fetch_remote().await.map_err(map_remote_error)?;
    Ok(data)
}

#[tokio::main]
async fn main() {
    match service_layer().await {
        Ok(v) => println!("OK: {}", v),
        Err(e) => {
            println!("MESSAGE: {}", e.kind.message());

            if let Some(ctx) = e.context {
                println!("SOURCE: {}", ctx.source);
            }
        }
    }
}
