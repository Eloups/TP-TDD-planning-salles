use std::fmt::Display;

/// Error enum for PositiveNumber struct
#[derive(Debug, Clone, PartialEq)]
pub enum PositiveNumberError {
    NumberIsZero,
}

impl Display for PositiveNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumberIsZero => write!(f, "Number can't be zero."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_is_zero_to_string_test() {
        let error = PositiveNumberError::NumberIsZero;
        assert_eq!(error.to_string(), "Number can't be zero.");
    }
}
