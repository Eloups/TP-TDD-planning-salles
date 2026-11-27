use sqlx::SqlitePool;

use crate::database::errors::DatabaseError;
use crate::entities::room::Room;

/// A room as in database
#[derive(Debug, Clone, PartialEq)]
pub struct RoomRecord {
    pub id: i64,
    pub room: Room,
}

#[derive(sqlx::FromRow)]
struct RoomRow {
    id: i64,
    name: String,
    max_capacity: u32,
}

impl TryFrom<RoomRow> for RoomRecord {
    type Error = DatabaseError;

    fn try_from(row: RoomRow) -> Result<Self, Self::Error> {
        let room = Room::new(row.name, row.max_capacity)
            .map_err(|error| DatabaseError::InvalidData(error.to_string()))?;
        Ok(Self { id: row.id, room })
    }
}

/// CRUD access to the `rooms` table
#[derive(Debug, Clone)]
pub struct RoomRepository {
    pool: SqlitePool,
}

impl RoomRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Inserts a room and returns it with its generated id
    pub async fn insert(&self, room: &Room) -> Result<RoomRecord, DatabaseError> {
        let id = sqlx::query("INSERT INTO rooms (name, max_capacity) VALUES (?, ?)")
            .bind(room.get_name())
            .bind(room.get_max_capacity().get())
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        Ok(RoomRecord {
            id,
            room: room.clone(),
        })
    }

    /// Finds a room by id, `None` if it does not exist
    pub async fn find_by_id(&self, id: i64) -> Result<Option<RoomRecord>, DatabaseError> {
        sqlx::query_as::<_, RoomRow>("SELECT id, name, max_capacity FROM rooms WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .map(RoomRecord::try_from)
            .transpose()
    }

    /// Lists every room, ordered by id
    pub async fn find_all(&self) -> Result<Vec<RoomRecord>, DatabaseError> {
        sqlx::query_as::<_, RoomRow>("SELECT id, name, max_capacity FROM rooms ORDER BY id")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(RoomRecord::try_from)
            .collect()
    }

    /// Replaces the name and capacity of an existing room
    pub async fn update(&self, id: i64, room: &Room) -> Result<RoomRecord, DatabaseError> {
        let result = sqlx::query("UPDATE rooms SET name = ?, max_capacity = ? WHERE id = ?")
            .bind(room.get_name())
            .bind(room.get_max_capacity().get())
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(RoomRecord {
            id,
            room: room.clone(),
        })
    }

    /// Deletes a room (and its bookings, through ON DELETE CASCADE)
    pub async fn delete(&self, id: i64) -> Result<(), DatabaseError> {
        let result = sqlx::query("DELETE FROM rooms WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DatabaseError::NotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::Database;

    async fn repository() -> RoomRepository {
        Database::in_memory().await.unwrap().rooms()
    }

    fn room(name: &str, max_capacity: u32) -> Room {
        Room::new(String::from(name), max_capacity).unwrap()
    }

    #[tokio::test]
    async fn insert_room_test() {
        // Arrange
        let repository = repository().await;
        let room = room("Salle A", 10);

        // Act
        let result = repository.insert(&room).await;

        // Assert
        let record = result.unwrap();
        assert_eq!(record.room, room);
        assert!(record.id > 0);
    }

    #[tokio::test]
    async fn inserted_rooms_get_different_ids_test() {
        let repository = repository().await;

        let first = repository.insert(&room("Salle A", 10)).await.unwrap();
        let second = repository.insert(&room("Salle B", 5)).await.unwrap();

        assert_ne!(first.id, second.id);
    }

    #[tokio::test]
    async fn find_room_by_id_test() {
        // Arrange
        let repository = repository().await;
        let inserted = repository.insert(&room("Salle A", 10)).await.unwrap();

        // Act
        let found = repository.find_by_id(inserted.id).await.unwrap();

        // Assert
        assert_eq!(found, Some(inserted));
    }

    #[tokio::test]
    async fn find_unknown_room_returns_none_test() {
        let repository = repository().await;

        let found = repository.find_by_id(42).await.unwrap();

        assert_eq!(found, None);
    }

    #[tokio::test]
    async fn find_all_rooms_test() {
        // Arrange
        let repository = repository().await;
        let first = repository.insert(&room("Salle A", 10)).await.unwrap();
        let second = repository.insert(&room("Salle B", 5)).await.unwrap();

        // Act
        let rooms = repository.find_all().await.unwrap();

        // Assert
        assert_eq!(rooms, vec![first, second]);
    }

    #[tokio::test]
    async fn find_all_rooms_on_empty_database_test() {
        let repository = repository().await;

        let rooms = repository.find_all().await.unwrap();

        assert!(rooms.is_empty());
    }

    #[tokio::test]
    async fn update_room_test() {
        // Arrange
        let repository = repository().await;
        let inserted = repository.insert(&room("Salle A", 10)).await.unwrap();
        let new_room = room("Salle A bis", 20);

        // Act
        let updated = repository.update(inserted.id, &new_room).await.unwrap();

        // Assert
        assert_eq!(updated.room, new_room);
        assert_eq!(
            repository.find_by_id(inserted.id).await.unwrap(),
            Some(updated)
        );
    }

    #[tokio::test]
    async fn update_unknown_room_fails_test() {
        let repository = repository().await;

        let result = repository.update(42, &room("Salle A", 10)).await;

        assert!(matches!(result, Err(DatabaseError::NotFound)));
    }

    #[tokio::test]
    async fn delete_room_test() {
        // Arrange
        let repository = repository().await;
        let inserted = repository.insert(&room("Salle A", 10)).await.unwrap();

        // Act
        let result = repository.delete(inserted.id).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(repository.find_by_id(inserted.id).await.unwrap(), None);
    }

    #[tokio::test]
    async fn delete_unknown_room_fails_test() {
        let repository = repository().await;

        let result = repository.delete(42).await;

        assert!(matches!(result, Err(DatabaseError::NotFound)));
    }

    #[tokio::test]
    async fn invalid_stored_room_is_rejected_test() {
        // Arrange : a row inserted without going through the Room validation
        let database = Database::in_memory().await.unwrap();
        sqlx::raw_sql("PRAGMA ignore_check_constraints = ON; INSERT INTO rooms (id, name, max_capacity) VALUES (1, '', 3);")
            .execute(&database.rooms().pool)
            .await
            .unwrap();

        // Act
        let result = database.rooms().find_by_id(1).await;

        // Assert
        assert!(matches!(result, Err(DatabaseError::InvalidData(_))));
    }
}
