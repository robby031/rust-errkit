use rust_errkit::{
    AppError, AppResult,
    kind::{ErrorKind, ErrorReason},
};

async fn fetch_remote() -> Result<String, &'static str> {
    Err("timeout")
}

async fn service_layer() -> AppResult<String> {
    let data = fetch_remote()
        .await
        .map_err(|_| ErrorKind::network(ErrorReason::Timeout).into())?;
    Ok(data)
}

#[tokio::main]
async fn main() {
    if let Err(e) = service_layer().await {
        println!("ERROR REPORT:\n{}", e);

        if e.kind.reason == ErrorReason::Timeout {
            println!("Action: Retry connection...");
        }
    }
}
