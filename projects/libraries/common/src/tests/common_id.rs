#[cfg(test)]
mod tests {
    use crate::{CommonID, custom_uuid::Id128};

    #[test]
    fn test_is_valid_with_zero_bytes() {
        let zero_id = Id128::from_bytes_unchecked([0u8; 16]);
        assert!(!CommonID::is_valid(zero_id));
    }
}
