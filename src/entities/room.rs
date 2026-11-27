use crate::entities::errors::room_error::RoomError;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Room {
    name: String,
    max_capacity: u32,
}

impl Room {
    /// Room constructor
    pub fn new(name: String, max_capacity: u32) -> Result<Self, RoomError> {
        if name.is_empty() {
            return Err(RoomError::EmptyNameError);
        }
        if max_capacity == 0 {
            return Err(RoomError::MaxCapacityError);
        }

        Ok(Self {
            name: name,
            max_capacity: max_capacity,
        })
    }

    /// Getter for the room name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Getter for the room max capacity
    pub fn get_max_capacity(&self) -> u32 {
        self.max_capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_room_test() {
        // Arrange
        let name = String::from("create room");
        let max_capacity = 4;

        // Act
        let room = Room::new(name.clone(), max_capacity);
        assert!(room.is_ok());

        // Assert
        assert_eq!(
            room.unwrap(),
            Room {
                name: name,
                max_capacity: max_capacity
            }
        );
    }

    #[test]
    fn cannot_create_room_with_empty_name_test() {
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
    fn cannot_create_room_with_0_capacity_test() {
        // Arrange
        let name = String::from("0 capacity room");
        let max_capacity = 0;

        // Act
        let room = Room::new(name.clone(), max_capacity);
        
        // Assert
        assert!(room.is_err());
        assert_eq!(room.unwrap_err(), RoomError::MaxCapacityError);
    }

    #[test]
    fn get_room_name_test() {
        // Arrange
        let name = String::from("no name");
        let max_capacity = 1;

        // Act
        let room = Room::new(name.clone(), max_capacity);
        
        // Assert
        assert!(room.is_ok());
        assert_eq!(room.unwrap().get_name(), name);
    }

    #[test]
    fn get_room_max_capacity_test() {
        // Arrange
        let name = String::from("get room capacity");
        let max_capacity = u32::max_value();

        // Act
        let room = Room::new(name.clone(), max_capacity);

        // Assert
        assert!(room.is_ok());
        assert_eq!(room.unwrap().get_max_capacity(), max_capacity);
    }
}
