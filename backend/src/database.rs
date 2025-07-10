use sea_orm::{DatabaseConnection, DbErr};

// TODO: change sqlite to real database instance later
const DB_URL: &str = "sqlite:dev.sqlite?mode=rwc";

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let db = sea_orm::Database::connect(DB_URL).await?;
    Ok(db)
}
