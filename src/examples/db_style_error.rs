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
        msg: "unique constraint violation".to_string(),
    })
}

fn map_db_error(e: DbError) -> AppError {
    AppError::from(ErrorKind::db(ErrorReason::Unexpected)).with_context("sqlx", Some(e.msg))
}

fn main() {
    let result = fake_db_call().map_err(map_db_error);

    match result {
        Ok(v) => println!("{}", v),
        Err(e) => {
            println!("MESSAGE: {}", e.kind.message());

            if let Some(ctx) = e.context {
                println!("DEBUG: {:?}", ctx.details);
            }
        }
    }
}
