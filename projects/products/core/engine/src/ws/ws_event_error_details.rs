// projects/products/core/engine/src/ws/ws_event_error_details.rs
// Define a new struct to encapsulate error details
pub(crate) struct WsEventErrorDetails {
    pub(crate) http_code: u16,
    pub(crate) code: i32,
    pub(crate) message: String,
}
