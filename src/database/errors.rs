use std::fmt::Display;

/// Error enum for the database layer
#[derive(Debug)]
pub enum DatabaseError {
    /// No row matches the requested id
    NotFound,
    /// A booking references a room that does not exist
    UnknownRoom,
    /// Data rejected by a constraint, or a stored row that is no longer valid
    InvalidData(String),
    /// Any other error coming from sqlx (connection, I/O, SQL...)
    Sqlx(sqlx::Error),
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Entity not found."),
            Self::UnknownRoom => write!(f, "The room does not exist."),
            Self::InvalidData(reason) => write!(f, "Invalid data : {reason}"),
            Self::Sqlx(error) => write!(f, "Database error : {error}"),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_error) = &error {
            if db_error.is_foreign_key_violation() {
                return Self::UnknownRoom;
            }
            if db_error.is_check_violation() {
                return Self::InvalidData(db_error.message().to_string());
            }
        }
        Self::Sqlx(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_to_string_test() {
        assert_eq!(DatabaseError::NotFound.to_string(), "Entity not found.");
    }

    #[test]
    fn unknown_room_to_string_test() {
        assert_eq!(
            DatabaseError::UnknownRoom.to_string(),
            "The room does not exist."
        );
    }

    #[test]
    fn invalid_data_to_string_test() {
        let error = DatabaseError::InvalidData(String::from("bad capacity"));

        assert_eq!(error.to_string(), "Invalid data : bad capacity");
    }

    #[test]
    fn row_not_found_is_converted_to_sqlx_error_test() {
        let error = DatabaseError::from(sqlx::Error::RowNotFound);

        assert!(matches!(error, DatabaseError::Sqlx(sqlx::Error::RowNotFound)));
    }
}
