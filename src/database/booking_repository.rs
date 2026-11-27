use chrono::{NaiveDate, NaiveDateTime};
use sqlx::SqlitePool;

use crate::database::errors::DatabaseError;

/// Booking from database structure
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct BookingRecord {
    pub id: i64,
    pub room_id: i64,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub nb_people: u32,
}

#[derive(Debug, Clone)]
pub struct BookingRepository {
    pool: SqlitePool,
}

impl BookingRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Inserts a booking and returns it with its generated id.
    pub async fn insert(
        &self,
        room_id: i64,
        start_at: NaiveDateTime,
        end_at: NaiveDateTime,
        nb_people: u32,
    ) -> Result<BookingRecord, DatabaseError> {
        let id = sqlx::query(
            "INSERT INTO bookings (room_id, start_at, end_at, nb_people) VALUES (?, ?, ?, ?)",
        )
        .bind(room_id)
        .bind(start_at)
        .bind(end_at)
        .bind(nb_people)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(BookingRecord {
            id,
            room_id,
            start_at,
            end_at,
            nb_people,
        })
    }

    /// Finds a booking by id, `None` if it does not exist
    pub async fn find_by_id(&self, id: i64) -> Result<Option<BookingRecord>, DatabaseError> {
        let booking = sqlx::query_as(
            "SELECT id, room_id, start_at, end_at, nb_people FROM bookings WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(booking)
    }

    /// Lists the bookings of a room, ordered by start date
    pub async fn find_by_room(&self, room_id: i64) -> Result<Vec<BookingRecord>, DatabaseError> {
        let bookings = sqlx::query_as(
            "SELECT id, room_id, start_at, end_at, nb_people FROM bookings WHERE room_id = ? ORDER BY start_at, id",
        )
        .bind(room_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(bookings)
    }

    /// Lists the bookings that cover at least part of `day`
    pub async fn find_by_day(&self, day: NaiveDate) -> Result<Vec<BookingRecord>, DatabaseError> {
        let day_start = day.and_hms_opt(0, 0, 0).expect("midnight is a valid time");
        let next_day_start = day_start + chrono::Duration::days(1);
        self.find_in_range(None, day_start, next_day_start).await
    }

    /// Lists the bookings of `room_id` that overlap `[start_at, end_at[`.
    pub async fn find_overlapping(
        &self,
        room_id: i64,
        start_at: NaiveDateTime,
        end_at: NaiveDateTime,
    ) -> Result<Vec<BookingRecord>, DatabaseError> {
        self.find_in_range(Some(room_id), start_at, end_at).await
    }

    async fn find_in_range(
        &self,
        room_id: Option<i64>,
        start_at: NaiveDateTime,
        end_at: NaiveDateTime,
    ) -> Result<Vec<BookingRecord>, DatabaseError> {
        let bookings = sqlx::query_as(
            "SELECT id, room_id, start_at, end_at, nb_people FROM bookings
             WHERE (?1 IS NULL OR room_id = ?1) AND start_at < ?2 AND end_at > ?3
             ORDER BY start_at, id",
        )
        .bind(room_id)
        .bind(end_at)
        .bind(start_at)
        .fetch_all(&self.pool)
        .await?;
        Ok(bookings)
    }

    /// Deletes a booking
    pub async fn delete(&self, id: i64) -> Result<(), DatabaseError> {
        let result = sqlx::query("DELETE FROM bookings WHERE id = ?")
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
    use crate::entities::room::Room;

    /// Parses "YYYY-MM-DD HH:MM"
    fn dt(value: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M").unwrap()
    }

    fn day(value: &str) -> NaiveDate {
        NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap()
    }

    /// In-memory database with one room, returns (database, room id)
    async fn setup() -> (Database, i64) {
        let database = Database::in_memory().await.unwrap();
        let room = database
            .rooms()
            .insert(&Room::new(String::from("Salle A"), 10).unwrap())
            .await
            .unwrap();
        (database, room.id)
    }

    #[tokio::test]
    async fn insert_booking_test() {
        // Arrange
        let (database, room_id) = setup().await;

        // Act
        let result = database
            .bookings()
            .insert(room_id, dt("2026-11-24 09:00"), dt("2026-11-24 10:00"), 4)
            .await;

        // Assert
        let booking = result.unwrap();
        assert!(booking.id > 0);
        assert_eq!(booking.room_id, room_id);
        assert_eq!(booking.start_at, dt("2026-11-24 09:00"));
        assert_eq!(booking.end_at, dt("2026-11-24 10:00"));
        assert_eq!(booking.nb_people, 4);
    }

    #[tokio::test]
    async fn insert_booking_for_unknown_room_fails_test() {
        let (database, _) = setup().await;

        let result = database
            .bookings()
            .insert(42, dt("2026-11-24 09:00"), dt("2026-11-24 10:00"), 4)
            .await;

        assert!(matches!(result, Err(DatabaseError::UnknownRoom)));
    }

    #[tokio::test]
    async fn insert_booking_with_start_after_end_fails_test() {
        let (database, room_id) = setup().await;

        let result = database
            .bookings()
            .insert(room_id, dt("2026-11-24 10:00"), dt("2026-11-24 09:00"), 4)
            .await;

        assert!(matches!(result, Err(DatabaseError::InvalidData(_))));
    }

    #[tokio::test]
    async fn find_booking_by_id_test() {
        // Arrange
        let (database, room_id) = setup().await;
        let inserted = database
            .bookings()
            .insert(room_id, dt("2026-11-24 09:00"), dt("2026-11-24 10:00"), 4)
            .await
            .unwrap();

        // Act
        let found = database.bookings().find_by_id(inserted.id).await.unwrap();

        // Assert
        assert_eq!(found, Some(inserted));
    }

    #[tokio::test]
    async fn find_unknown_booking_returns_none_test() {
        let (database, _) = setup().await;

        let found = database.bookings().find_by_id(42).await.unwrap();

        assert_eq!(found, None);
    }

    #[tokio::test]
    async fn find_bookings_by_room_test() {
        // Arrange
        let (database, room_id) = setup().await;
        let other_room = database
            .rooms()
            .insert(&Room::new(String::from("Salle B"), 5).unwrap())
            .await
            .unwrap();
        let bookings = database.bookings();
        let afternoon = bookings
            .insert(room_id, dt("2026-11-24 14:00"), dt("2026-11-24 15:00"), 2)
            .await
            .unwrap();
        let morning = bookings
            .insert(room_id, dt("2026-11-24 09:00"), dt("2026-11-24 10:00"), 2)
            .await
            .unwrap();
        bookings
            .insert(
                other_room.id,
                dt("2026-11-24 09:00"),
                dt("2026-11-24 10:00"),
                2,
            )
            .await
            .unwrap();

        // Act
        let found = bookings.find_by_room(room_id).await.unwrap();

        // Assert : only this room, sorted by start date
        assert_eq!(found, vec![morning, afternoon]);
    }

    #[tokio::test]
    async fn find_bookings_by_day_test() {
        // Arrange
        let (database, room_id) = setup().await;
        let bookings = database.bookings();
        let previous_day = bookings
            .insert(room_id, dt("2026-11-23 09:00"), dt("2026-11-23 10:00"), 2)
            .await
            .unwrap();
        let same_day = bookings
            .insert(room_id, dt("2026-11-24 09:00"), dt("2026-11-24 10:00"), 2)
            .await
            .unwrap();
        let spanning = bookings
            .insert(room_id, dt("2026-11-23 18:00"), dt("2026-11-25 08:00"), 2)
            .await
            .unwrap();
        let next_day_midnight = bookings
            .insert(room_id, dt("2026-11-25 00:00"), dt("2026-11-25 01:00"), 2)
            .await
            .unwrap();
        let ends_at_midnight = bookings
            .insert(room_id, dt("2026-11-23 23:00"), dt("2026-11-24 00:00"), 2)
            .await
            .unwrap();

        // Act
        let found = bookings.find_by_day(day("2026-11-24")).await.unwrap();

        // Assert
        assert_eq!(found, vec![spanning, same_day]);
        assert!(!found.contains(&previous_day));
        assert!(!found.contains(&next_day_midnight));
        assert!(!found.contains(&ends_at_midnight));
    }

    #[tokio::test]
    async fn find_overlapping_bookings_test() {
        // Arrange : existing booking 10:00 - 12:00
        let (database, room_id) = setup().await;
        let bookings = database.bookings();
        let existing = bookings
            .insert(room_id, dt("2026-11-24 10:00"), dt("2026-11-24 12:00"), 2)
            .await
            .unwrap();

        let overlapping_ranges = [
            ("2026-11-24 09:00", "2026-11-24 11:00"), // overlaps the start
            ("2026-11-24 11:00", "2026-11-24 13:00"), // overlaps the end
            ("2026-11-24 10:30", "2026-11-24 11:30"), // inside
            ("2026-11-24 09:00", "2026-11-24 13:00"), // contains it
            ("2026-11-24 10:00", "2026-11-24 12:00"), // same range
        ];

        // Act + Assert
        for (start, end) in overlapping_ranges {
            let found = bookings
                .find_overlapping(room_id, dt(start), dt(end))
                .await
                .unwrap();
            assert_eq!(found, vec![existing.clone()], "{start} -> {end}");
        }
    }

    #[tokio::test]
    async fn adjacent_bookings_do_not_overlap_test() {
        // Arrange : existing booking 10:00 - 12:00
        let (database, room_id) = setup().await;
        let bookings = database.bookings();
        bookings
            .insert(room_id, dt("2026-11-24 10:00"), dt("2026-11-24 12:00"), 2)
            .await
            .unwrap();

        // Act
        let before = bookings
            .find_overlapping(room_id, dt("2026-11-24 09:00"), dt("2026-11-24 10:00"))
            .await
            .unwrap();
        let after = bookings
            .find_overlapping(room_id, dt("2026-11-24 12:00"), dt("2026-11-24 13:00"))
            .await
            .unwrap();

        // Assert
        assert!(before.is_empty());
        assert!(after.is_empty());
    }

    #[tokio::test]
    async fn bookings_of_another_room_do_not_overlap_test() {
        // Arrange
        let (database, room_id) = setup().await;
        let other_room = database
            .rooms()
            .insert(&Room::new(String::from("Salle B"), 5).unwrap())
            .await
            .unwrap();
        database
            .bookings()
            .insert(
                other_room.id,
                dt("2026-11-24 10:00"),
                dt("2026-11-24 12:00"),
                2,
            )
            .await
            .unwrap();

        // Act
        let found = database
            .bookings()
            .find_overlapping(room_id, dt("2026-11-24 10:00"), dt("2026-11-24 12:00"))
            .await
            .unwrap();

        // Assert
        assert!(found.is_empty());
    }

    #[tokio::test]
    async fn delete_booking_test() {
        // Arrange
        let (database, room_id) = setup().await;
        let inserted = database
            .bookings()
            .insert(room_id, dt("2026-11-24 09:00"), dt("2026-11-24 10:00"), 4)
            .await
            .unwrap();

        // Act
        let result = database.bookings().delete(inserted.id).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(
            database.bookings().find_by_id(inserted.id).await.unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn delete_unknown_booking_fails_test() {
        let (database, _) = setup().await;

        let result = database.bookings().delete(42).await;

        assert!(matches!(result, Err(DatabaseError::NotFound)));
    }

    #[tokio::test]
    async fn deleting_a_room_deletes_its_bookings_test() {
        // Arrange
        let (database, room_id) = setup().await;
        let booking = database
            .bookings()
            .insert(room_id, dt("2026-11-24 09:00"), dt("2026-11-24 10:00"), 4)
            .await
            .unwrap();

        // Act
        database.rooms().delete(room_id).await.unwrap();

        // Assert
        assert_eq!(
            database.bookings().find_by_id(booking.id).await.unwrap(),
            None
        );
    }
}
