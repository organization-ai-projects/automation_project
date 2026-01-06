pub fn init() {
    println!("Initializing common library...");
}

pub mod common_id;
pub mod error_type;
pub mod name;
pub mod timestamp;
pub mod utils;

pub use common_id::CommonID;
pub use error_type::ErrorType;
