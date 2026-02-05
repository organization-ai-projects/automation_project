// projects/products/core/engine/src/runtime/backend_connection.rs
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub(crate) struct BackendConnection {
    pub(crate) sender: UnboundedSender<String>,
}
