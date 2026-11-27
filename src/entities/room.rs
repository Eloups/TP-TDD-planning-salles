use crate::entities::{errors::room_error::RoomError, types::positive_number::PositiveNumber};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Room {
    name: String,
    max_capacity: PositiveNumber,
}

impl Room {
    /// Room constructor
    pub fn new(name: String, max_capacity: u32) -> Result<Self, RoomError> {
        if name.is_empty() {
            return Err(RoomError::EmptyNameError);
        }

        Ok(Self {
            name,
            max_capacity: PositiveNumber::new(max_capacity)?,
        })
    }

    /// Getter for the room name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Getter for the room max capacity
    pub fn get_max_capacity(&self) -> PositiveNumber {
        self.max_capacity.clone()
    }

    /// Setter for the room name
    pub fn set_name(&mut self, name: String) -> Result<(), RoomError> {
        if name.is_empty() {
            return Err(RoomError::EmptyNameError);
        }
        self.name = name;
        Ok(())
    }

    /// Setter for the room name
    pub fn set_max_capacity(&mut self, max_capacity: u32) -> Result<(), RoomError> {
        self.max_capacity = PositiveNumber::new(max_capacity)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::types::errors::positive_number_error::PositiveNumberError;

    use super::*;

    #[test]
    fn create_room_test() {
        // Arrange
        let name = String::from("create room");
        let max_capacity = 4;

        // Act
        let room = Room::new(name.clone(), max_capacity);

        // Assert
        assert!(room.is_ok());
        assert_eq!(
            room.unwrap(),
            Room {
                name: name,
                max_capacity: PositiveNumber::new(max_capacity).unwrap()
            }
        );
    }

    #[test]
    fn create_room_with_empty_name_failure_test() {
        // Arrange
        let name = String::from("");
        let max_capacity = 10;

        // Act
        let room = Room::new(name.clone(), max_capacity);

        // Assert
        assert!(room.is_err());
        assert_eq!(room.unwrap_err(), RoomError::EmptyNameError);
    }

    #[test]
    fn create_room_with_0_capacity_failure_test() {
        // Arrange
        let name = String::from("0 capacity room");
        let max_capacity = 0;

        // Act
        let room = Room::new(name.clone(), max_capacity);

        // Assert
        assert!(room.is_err());
        assert_eq!(
            room.unwrap_err(),
            RoomError::PositiveNumberError(PositiveNumberError::NumberIsZero)
        );
    }

    #[test]
    fn get_room_name_test() {
        // Arrange
        let name = String::from("no name");
        let max_capacity = 1;

        // Act
        let room = Room::new(name.clone(), max_capacity);
        assert!(room.is_ok());

        // Assert
        assert_eq!(room.unwrap().get_name(), name);
    }

    #[test]
    fn get_room_max_capacity_test() {
        // Arrange
        let name = String::from("get room capacity");
        let max_capacity = 2;

        // Act
        let room = Room::new(name.clone(), max_capacity);
        assert!(room.is_ok());

        // Assert
        assert_eq!(
            room.unwrap().get_max_capacity(),
            PositiveNumber::new(max_capacity).unwrap()
        );
    }

    #[test]
    fn change_room_name_test() {
        // Arrange
        let name = String::from("original room name");
        let max_capacity = 3;

        let room = Room::new(name.clone(), max_capacity);
        assert!(room.is_ok());
        let mut room = room.unwrap();

        let new_name = String::from("new room name");

        // Act
        let result = room.set_name(new_name.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(room.get_name(), new_name);
    }

    #[test]
    fn change_room_name_to_empty_name_failure_test() {
        // Arrange
        let name = String::from("cannot change set empty name");
        let max_capacity = 4;

        let room = Room::new(name.clone(), max_capacity);
        assert!(room.is_ok());
        let mut room = room.unwrap();

        let new_name = String::from("");

        // Act
        let result = room.set_name(new_name.clone());

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RoomError::EmptyNameError);
    }

    #[test]
    fn change_room_max_capacity_test() {
        // Arrange
        let name = String::from("change room capacity");
        let max_capacity = 5;

        let room = Room::new(name.clone(), max_capacity);
        assert!(room.is_ok());
        let mut room = room.unwrap();

        let new_max_capacity = 1;

        // Act
        let result = room.set_max_capacity(new_max_capacity.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(
            room.get_max_capacity(),
            PositiveNumber::new(new_max_capacity).unwrap()
        );
    }

    #[test]
    fn change_room_max_capacity_to_0_failure_test() {
        // Arrange
        let name = String::from("cannot change room capacity to 0");
        let max_capacity = 6;

        let room = Room::new(name.clone(), max_capacity);
        assert!(room.is_ok());
        let mut room = room.unwrap();

        let new_max_capacity = 0;

        // Act
        let result = room.set_max_capacity(new_max_capacity);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RoomError::PositiveNumberError(PositiveNumberError::NumberIsZero)
        );
    }
}
