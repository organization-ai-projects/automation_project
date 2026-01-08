// projects/libraries/common/src/common_id.rs
// Example: Define a common ID type
pub type CommonID = u64;

// projects/libraries/common/src/utils.rs
// Example: A utility function
pub fn is_valid_id(id: u64) -> bool {
    id > 0
}
