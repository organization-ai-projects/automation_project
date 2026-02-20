// projects/libraries/common/src/name.rs
// Define a common name type
pub type Name = String;

// Validate if a string is a valid name
pub fn is_valid_name(name: &str) -> bool {
    !name.trim().is_empty()
}
