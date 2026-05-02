use rust_errkit::{
    AppError,
    kind::{ErrorKind, ErrorReason},
};

#[derive(Debug)]
struct DbError {
    msg: String,
}

fn fake_db_call() -> Result<String, DbError> {
    Err(DbError {
        msg: "unique constraint violation on 'users.email'".to_string(),
    })
}

fn main() {
    let result = fake_db_call().map_err(|e| {
        AppError::from(ErrorKind::db(ErrorReason::AlreadyExists))
            .with_context("postgres_main", Some(e.msg))
    });

    if let Err(e) = result {
        println!("DATABASE FAILURE: {}", e);
    }
}
