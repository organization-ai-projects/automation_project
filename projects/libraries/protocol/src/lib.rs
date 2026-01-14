// projects/libraries/protocol/src/lib.rs
pub fn init() {
    println!("Initializing protocol library...");
}

pub mod apply_request;
pub mod apply_response;
pub mod command;
pub mod command_response;
pub mod command_type;
pub mod event;
pub mod event_type;
pub mod event_variant;
pub mod json;
pub mod log_level;
pub mod metadata;
pub mod payload;
pub mod preview_request;
pub mod preview_response;
pub mod protocol_error;
pub mod response_status;
pub mod validation_error;

pub use apply_request::ApplyRequest;
pub use apply_response::ApplyResponse;
pub use command::Command;
pub use command_response::CommandResponse;
pub use command_type::CommandType;
pub use event::Event;
pub use event_type::EventType;
pub use event_variant::EventVariant;
pub use json::{
    Json, JsonDeserializable, JsonError, JsonMap, JsonSerializable, from_json, from_json_owned,
    from_json_str, json, to_json, to_json_string, to_json_string_pretty,
};
// Re-export macros from common_json
pub use common_json::{pjson, pjson_key};
pub use log_level::LogLevel;
pub use metadata::Metadata;
pub use payload::Payload;
pub use preview_request::PreviewRequest;
pub use preview_response::PreviewResponse;
pub use protocol_error::ProtocolError;
pub use response_status::ResponseStatus;
pub use validation_error::ValidationError;
