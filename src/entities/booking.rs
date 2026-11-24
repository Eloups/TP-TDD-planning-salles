#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Booking {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_booking_test() {
        // TODO start + end + nbPersonne + Room
    }

    #[test]
    fn creation_with_unexisting_room_failure_test() {
        // TODO fail si room not found in DB
    }

    #[test]
    fn creation_with_to_much_people_failure_test() {
        // TODO fail si nbMaxPeople < nbPeople
    }

    #[test]
    fn creation_with_wrong_dates_failure_test() {
        // TODO fail si start > end
    }

    #[test]
    fn creation_with_wrong_start_date_failure_test() {
        // TODO fail si start < today
    }
}