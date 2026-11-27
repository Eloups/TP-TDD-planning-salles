pub mod booking_repository;
pub mod errors;
pub mod room_repository;

use std::path::Path;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use booking_repository::BookingRepository;
use errors::DatabaseError;
use room_repository::RoomRepository;

const SCHEMA: &str = include_str!("schema.sql");

#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Connection function
    pub async fn connect(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| DatabaseError::Sqlx(sqlx::Error::Io(error)))?;
        }

        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new().connect_with(options).await?;
        Self::init(pool).await
    }

    /// Opens a in-memory database for testing.
    pub async fn in_memory() -> Result<Self, DatabaseError> {
        let options = SqliteConnectOptions::new()
            .in_memory(true)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        Self::init(pool).await
    }

    async fn init(pool: SqlitePool) -> Result<Self, DatabaseError> {
        sqlx::raw_sql(SCHEMA).execute(&pool).await?;
        Ok(Self { pool })
    }

    /// Repository for the `rooms` table
    pub fn rooms(&self) -> RoomRepository {
        RoomRepository::new(self.pool.clone())
    }

    /// Repository for the `bookings` table
    pub fn bookings(&self) -> BookingRepository {
        BookingRepository::new(self.pool.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn in_memory_database_creates_tables_test() {
        let database = Database::in_memory().await.unwrap();

        let tables: Vec<String> = sqlx::query_scalar(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name IN ('rooms', 'bookings') ORDER BY name",
        )
        .fetch_all(&database.pool)
        .await
        .unwrap();

        assert_eq!(tables, vec!["bookings", "rooms"]);
    }

    #[tokio::test]
    async fn schema_can_be_applied_twice_test() {
        let database = Database::in_memory().await.unwrap();

        let result = Database::init(database.pool.clone()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn file_database_is_created_with_its_directory_test() {
        let directory =
            std::env::temp_dir().join(format!("tp_planning_salle_test_{}", std::process::id()));
        let path = directory.join("nested").join("test.db");

        let database = Database::connect(&path).await;

        assert!(database.is_ok());
        assert!(path.exists());

        database.unwrap().pool.close().await;
        std::fs::remove_dir_all(directory).unwrap();
    }
}
