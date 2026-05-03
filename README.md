# rust-errkit

[![Crates.io](https://img.shields.io/crates/v/rust-errkit?style=flat-square&color=orange&logo=rust)](https://crates.io/crates/rust-errkit)
[![Documentation](https://img.shields.io/docsrs/rust-errkit?style=flat-square&logo=docs.rs)](https://docs.rs/rust-errkit)
[![License](https://img.shields.io/crates/l/rust-errkit?style=flat-square&color=blue)](https://crates.io/crates/rust-errkit)
[![Downloads](https://img.shields.io/crates/d/rust-errkit?style=flat-square&color=darkgreen)](https://crates.io/crates/rust-errkit)
[![Support me on Ko-fi](https://img.shields.io/badge/Support%20me-Ko--fi-F16061?style=flat-square&logo=ko-fi&logoColor=white)](https://ko-fi.com/robby031)

**rust-errkit** adalah library error handling idiomatik untuk Rust yang memudahkan pembuatan, mapping, dan penanganan error secara terstruktur dan konsisten.

## Fitur Utama

- Definisi error domain dan reason yang jelas
- Konversi error eksternal ke error aplikasi
- Penambahan context pada error
- Trait untuk konversi Result ke AppResult
- Mudah diintegrasikan pada aplikasi dan service async

## Instalasi

Tambahkan ke `Cargo.toml` Anda:

```toml
[dependencies]
rust-errkit = "0.1.1"
```

## Penggunaan Dasar

### Membuat Error

```rust
use rust_errkit::{AppError, kind::{ErrorKind, ErrorReason}};

let err: AppError = ErrorKind::network(ErrorReason::Timeout).into();

println!("{}", err);
```

### Menambah Context pada Error

```rust
use rust_errkit::{AppError, kind::{ErrorKind, ErrorReason}};

let err = AppError::from(ErrorKind::db(ErrorReason::ConnectionFailed))
    .with_context("sqlx", Some("connection refused".to_string()));

println!("{}", err);

if let Some(ctx) = &err.context {
    println!("SOURCE: {}", ctx.source);
    println!("DETAILS: {:?}", ctx.details);
}
```

### Mapping Error Eksternal

```rust
use rust_errkit::{AppError, kind::{ErrorKind, ErrorReason}};

fn map_db_error(e: &str) -> AppError {
    AppError::from(ErrorKind::db(ErrorReason::Unexpected))
        .with_context("sqlx", Some(e.to_string()))
}
```

### Menggunakan AppResult dan ResultExt

```rust
use rust_errkit::{AppResult, AppError, kind::{ErrorKind, ErrorReason}};

fn might_fail(success: bool) -> Result<String, &'static str> {
    if success {
        Ok("OK".to_string())
    } else {
        Err("something went wrong")
    }
}

fn main() -> AppResult<()> {
    let value = might_fail(false)
        .map_err(|e| {
            AppError::from(ErrorKind::core(ErrorReason::Unexpected))
                .with_context("might_fail", Some(e.to_string()))
        })?;

    println!("{}", value);
    Ok(())
}
```

### Flow pada Async Service

```rust
use rust_errkit::{AppError, AppResult, kind::{ErrorKind, ErrorReason}};

async fn fetch_remote() -> Result<String, &'static str> {
    Err("timeout")
}

async fn service_layer() -> AppResult<String> {
    let data = fetch_remote().await
        .map_err(|_| ErrorKind::network(ErrorReason::Timeout).into())?;
    Ok(data)
}
```

## Struktur Error

- `AppError` : Error utama aplikasi
- `ErrorKind` : Kombinasi domain dan reason
- `ErrorDomain` : Kategori error (Core, Network, Database, IO, Auth, External)
- `ErrorReason` : Penyebab error (Timeout, NotFound, Unauthorized, dsb)
- `ErrorContext` : Informasi tambahan (source, details)

## Kontribusi

Pull request dan issue sangat diterima!

## Lisensi

Lihat [LICENSE](LICENSE) untuk detail lisensi.
