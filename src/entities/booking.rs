use crate::entities::{
    errors::booking_error::BookingError, types::positive_number::PositiveNumber,
};

use super::room::Room;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Booking {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    nb_people: PositiveNumber,
    room: Room,
}

impl Booking {
    pub fn new(
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        nb_people: u32,
        room: Room,
    ) -> Result<Self, BookingError> {
        let nb_people = PositiveNumber::new(nb_people)?;
        if nb_people > room.get_max_capacity() {
            return Err(BookingError::TooManyPeopleError);
        }
        if start_date > end_date {
            return Err(BookingError::StartDateError);
        }
        if start_date == end_date {
            return Err(BookingError::DurationError);
        }
        Ok(Self {
            start_date,
            end_date,
            nb_people,
            room,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::{
        errors::booking_error::BookingError,
        types::errors::positive_number_error::PositiveNumberError,
    };

    use super::*;
    use chrono::prelude::*;

    #[test]
    fn create_booking_test() {
        // Arrange
        let room_name = String::from("create booking");
        let room_max_capacity = 4;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date = Utc.with_ymd_and_hms(2027, 1, 1, 8, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2027, 1, 1, 12, 0, 0).unwrap();
        let nb_people = 2;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());

        // Assert
        assert!(booking.is_ok());
        assert_eq!(
            booking.unwrap(),
            Booking {
                start_date,
                end_date,
                nb_people: PositiveNumber::new(nb_people).unwrap(),
                room,
            }
        );
    }

    #[test]
    fn creation_with_0_peron_failure_test() {
        // Arrange
        let room_name = String::from("create booking for 0 person");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date = Utc.with_ymd_and_hms(2020, 2, 3, 18, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2020, 8, 4, 12, 3, 10).unwrap();
        let nb_people = 0;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());

        // Assert
        assert!(booking.is_err());
        assert_eq!(
            booking.unwrap_err(),
            BookingError::PositiveNumberError(PositiveNumberError::NumberIsZero)
        );
    }

    #[test]
    fn creation_with_to_much_people_failure_test() {
        // Arrange
        let room_name = String::from("create booking too much people");
        let room_max_capacity = 8;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date = Utc.with_ymd_and_hms(2020, 2, 3, 18, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2020, 2, 4, 12, 3, 10).unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());

        // Assert
        assert!(booking.is_err());
        assert_eq!(booking.unwrap_err(), BookingError::TooManyPeopleError);
    }

    #[test]
    fn creation_with_max_number_of_people_test() {
        // Arrange
        let room_name = String::from("create booking full room");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date = Utc.with_ymd_and_hms(2027, 1, 1, 8, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2027, 1, 1, 12, 0, 0).unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());

        // Assert
        assert!(booking.is_ok());
        assert_eq!(
            booking.unwrap(),
            Booking {
                start_date,
                end_date,
                nb_people: PositiveNumber::new(nb_people).unwrap(),
                room,
            }
        );
    }

    #[test]
    fn creation_with_wrong_dates_failure_test() {
        // Arrange
        let room_name = String::from("create booking full room");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date = Utc.with_ymd_and_hms(2027, 1, 1, 13, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2027, 1, 1, 12, 0, 0).unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());

        // Assert
        assert!(booking.is_err());
        assert_eq!(booking.unwrap_err(), BookingError::StartDateError);
        // TODO fail si start > end
    }

    #[test]
    fn creation_with_invalid_duration_failure_test() {
        // Arrange
        let room_name = String::from("create booking full room");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date = Utc.with_ymd_and_hms(2027, 1, 1, 12, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2027, 1, 1, 12, 0, 0).unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());

        // Assert
        assert!(booking.is_err());
        assert_eq!(booking.unwrap_err(), BookingError::DurationError);
        // TODO fail si start > end
    }

    #[test]
    fn creation_minimum_positive_duration_test() {
        // Arrange
        let room_name = String::from("create booking full room");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date = Utc.with_ymd_and_hms(2027, 1, 1, 12, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2027, 1, 1, 12, 0, 1).unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());

        // Assert
        assert!(booking.is_ok());
        assert_eq!(
            booking.unwrap(),
            Booking {
                start_date,
                end_date,
                nb_people: PositiveNumber::new(nb_people).unwrap(),
                room,
            }
        );
    }
}
