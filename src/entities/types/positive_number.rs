use crate::entities::types::errors::positive_number_error::PositiveNumberError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositiveNumber(u32);

impl PositiveNumber {
    /// Constructor of PositiveNumber
    pub fn new(number: u32) -> Result<Self, PositiveNumberError> {
        if number == 0 {
            return Err(PositiveNumberError::NumberIsZero);
        }

        Ok(Self(number))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_positive_number_test() {
        let number = 3;

        let result = PositiveNumber::new(number);
        assert!(result.is_ok());

        let positive_number = result.unwrap();
        assert_eq!(positive_number, PositiveNumber(3));
    }

    #[test]
    fn create_positive_number_with_zero_error_test() {
        let number = 0;

        let result = PositiveNumber::new(number);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error, PositiveNumberError::NumberIsZero);
    }
}
