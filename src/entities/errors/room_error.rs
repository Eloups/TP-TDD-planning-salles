use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum RoomError {
    MaxCapacityError,
    EmptyNameError,
}

impl Display for RoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MaxCapacityError => write!(f, "Max capacity should be positive"),
            Self::EmptyNameError => write!(f, "Name cannot be an empty string"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_capacity_error_to_string_test() {
        let error = RoomError::MaxCapacityError;

        assert_eq!(error.to_string(), "Max capacity should be positive");
    }

    #[test]
    fn empty_name_error_to_string_test() {
        let error = RoomError::EmptyNameError;

        assert_eq!(error.to_string(), "Name cannot be an empty string");
    }
}
