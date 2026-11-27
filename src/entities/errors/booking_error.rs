use std::fmt::Display;

use crate::entities::types::errors::positive_number_error::PositiveNumberError;

#[derive(Debug, Clone, PartialEq)]
pub enum BookingError {
    PositiveNumberError(PositiveNumberError),
    TooManyPeopleError,
    StartDateError,
    DurationError,
}

impl Display for BookingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PositiveNumberError(error) => {
                write!(f, "PositiveNumberError : {}", error.to_string())
            }
            Self::TooManyPeopleError => {
                write!(f, "Number of people can't exceed the room's max capcity")
            }
            Self::StartDateError => {
                write!(f, "Start date cannot be after en date")
            }
            Self::DurationError => {
                write!(f, "Duration is too short")
            }
        }
    }
}

impl From<PositiveNumberError> for BookingError {
    fn from(value: PositiveNumberError) -> Self {
        Self::PositiveNumberError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_number_error_to_string_test() {
        let error = BookingError::PositiveNumberError(PositiveNumberError::NumberIsZero);

        assert_eq!(
            error.to_string(),
            "PositiveNumberError : Number can't be zero."
        );
    }

    #[test]
    fn empty_name_error_to_string_test() {
        let error = BookingError::TooManyPeopleError;

        assert_eq!(
            error.to_string(),
            "Number of people can't exceed the room's max capcity"
        );
    }

    #[test]
    fn start_date_error_to_string_test() {
        let error = BookingError::StartDateError;

        assert_eq!(error.to_string(), "Start date cannot be after en date");
    }

    #[test]
    fn duration_error_to_string_test() {
        let error = BookingError::DurationError;

        assert_eq!(error.to_string(), "Duration is too short");
    }
}
