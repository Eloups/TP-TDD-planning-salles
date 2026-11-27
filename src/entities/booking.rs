use crate::entities::{
    errors::booking_error::BookingError, types::positive_number::PositiveNumber,
};

use super::room::Room;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Booking {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    nb_people: PositiveNumber,
    room: Room,
}

impl Booking {
    pub fn new(
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
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

    /// Getter for start date
    pub fn get_start_date(&self) -> NaiveDateTime {
        self.start_date
    }

    /// Getter for end date
    pub fn get_end_date(&self) -> NaiveDateTime {
        self.end_date
    }

    /// Getter for nb people
    pub fn get_nb_people(&self) -> u32 {
        self.nb_people.get()
    }

    /// Getter for room
    pub fn get_room(&self) -> Room {
        self.room.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::{
        errors::booking_error::BookingError,
        types::errors::positive_number_error::PositiveNumberError,
    };

    use super::*;

    #[test]
    fn create_booking_test() {
        // Arrange
        let room_name = String::from("create booking");
        let room_max_capacity = 4;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 08:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
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

        let start_date =
            NaiveDateTime::parse_from_str("03/02/2020 18:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("04/08/2020 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
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

        let start_date =
            NaiveDateTime::parse_from_str("03/02/2020 18:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("04/02/2020 12:03:10", "%d/%m/%Y %H:%M:%S").unwrap();
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

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 08:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
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
        let room_name = String::from("create booking wrong dates");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 13:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
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
        let room_name = String::from("create booking duration too short");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
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
        let room_name = String::from("create booking minimum duration");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:01", "%d/%m/%Y %H:%M:%S").unwrap();
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
    fn get_start_date_test() {
        // Arrange
        let room_name = String::from("create booking start date");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:01", "%d/%m/%Y %H:%M:%S").unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());
        assert!(booking.is_ok());
        let booking = booking.unwrap();

        // Assert
        assert_eq!(booking.get_start_date(), start_date);
    }

    #[test]
    fn get_end_date_test() {
        // Arrange
        let room_name = String::from("create booking end date");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:01", "%d/%m/%Y %H:%M:%S").unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());
        assert!(booking.is_ok());
        let booking = booking.unwrap();

        // Assert
        assert_eq!(booking.get_end_date(), end_date);
    }

    #[test]
    fn get_nb_people_test() {
        // Arrange
        let room_name = String::from("create booking nb people");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:01", "%d/%m/%Y %H:%M:%S").unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());
        assert!(booking.is_ok());
        let booking = booking.unwrap();

        // Assert
        assert_eq!(booking.get_nb_people(), nb_people);
    }

    #[test]
    fn get_room_test() {
        // Arrange
        let room_name = String::from("create booking nb people");
        let room_max_capacity = 9;

        let room = Room::new(room_name.clone(), room_max_capacity);
        assert!(room.is_ok());
        let room = room.unwrap();

        let start_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str("01/01/2027 12:00:01", "%d/%m/%Y %H:%M:%S").unwrap();
        let nb_people = 9;

        // Act
        let booking = Booking::new(start_date, end_date, nb_people, room.clone());
        assert!(booking.is_ok());
        let booking = booking.unwrap();

        // Assert
        assert_eq!(booking.get_room(), room);
    }
}
