#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Room {
    name: String,
    max_capacity: u32
}

impl Room {
    /// Room constructor
    pub fn new(name: String, max_capacity: u32) -> Self {
        Self {
            name: name,
            max_capacity: max_capacity
        }
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
        let name = String::from("test");
        let max_capacity = 4;

        // Act
        let room = Room::new(name, max_capacity);

        // Assert
        assert_eq!(room.get_name(), "test");
        assert_eq!(room.get_max_capacity(), 4);
    }
}