// projects/libraries/common/src/lib.rs
pub mod common_id;
pub mod custom_uuid;
pub mod error_type;
pub mod id_error;
pub mod name;
pub mod string_manipulate;
mod tests;

pub use common_id::CommonID;
pub use common_time::timestamp_utils::format_timestamp;
pub use custom_uuid::Id128;
pub use error_type::ErrorType;
pub use id_error::IdError;
pub use name::Name;
pub use string_manipulate::{trim_lossy, truncate_utf8};
