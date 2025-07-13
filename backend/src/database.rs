use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

// TODO: change sqlite to real database instance later
const DB_URL: &str = "sqlite:dev.sqlite?mode=rwc";

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    Ok(SqlitePoolOptions::default().connect(DB_URL).await?)
}
