use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::{Error as SqlxError, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use thiserror::Error as ThisError;
use crate::utils::load_config::ConfigApp;

pub type DbPool = Pool<Postgres>;
pub type Result<T> = std::result::Result<T, DbError>;

lazy_static! {
    static ref DB_POOL: AsyncOnce<Result<DbPool>> = AsyncOnce::new(
        async {create_connection().await}
    );
}

pub async fn create_connection() -> Result<DbPool> {
    let db_config = ConfigApp::load_db().db;

    PgPoolOptions::new()
        .max_connections(db_config.max_connection)
        .connect(&db_config.db_url)
        .await.map_err(DbError::from)
}

pub async fn get_connection() -> Result<DbPool> {
    DB_POOL.get().await.clone()
}

#[derive(Debug, Clone, ThisError)]
pub enum DbError {
    #[error("An error occurred during database interaction. {0}")]
    DatabaseError(String)
}

impl From<SqlxError> for DbError {
    fn from(value: SqlxError) -> Self {
        match value.as_database_error() {
            None => {
                eprintln!("error: {:?}", value);
                DbError::DatabaseError(String::from("Unrecognized database error!"))
            }
            Some(error_message) => DbError::DatabaseError(error_message.to_string())
        }
    }
}