mod models;

use sqlx::migrate::MigrateError;
use sqlx::MySql;
use std::sync::Arc;
use tracing::error;

pub type Database = sqlx::Pool<MySql>;

pub async fn init_database(database_url: &str) -> Database {
    match Database::connect(database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            error!(target: "Database", "Error connecting to database: {e}");
            std::process::exit(1);
        }
    }
}

pub async fn run_migrations(db: &Database) -> Result<(), MigrateError> {
    match sqlx::migrate!("../../migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to run database migrations: {}", e);
            Err(e)
        }
    }
}
