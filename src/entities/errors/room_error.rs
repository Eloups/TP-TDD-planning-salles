use crate::entities::types::errors::positive_number_error::PositiveNumberError;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum RoomError {
    EmptyNameError,
    PositiveNumberError(PositiveNumberError),
}

impl Display for RoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyNameError => write!(f, "Name cannot be an empty string"),
            Self::PositiveNumberError(error) => {
                write!(f, "PositiveNumberError : {}", error.to_string())
            }
        }
    }
}

impl From<PositiveNumberError> for RoomError {
    fn from(value: PositiveNumberError) -> Self {
        Self::PositiveNumberError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name_error_to_string_test() {
        let error = RoomError::EmptyNameError;

        assert_eq!(error.to_string(), "Name cannot be an empty string");
    }

    #[test]
    fn positive_number_error_to_string_test() {
        let error = RoomError::PositiveNumberError(PositiveNumberError::NumberIsZero);

        assert_eq!(
            error.to_string(),
            "PositiveNumberError : Number can't be zero."
        );
    }
}
