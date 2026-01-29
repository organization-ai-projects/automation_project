// projects/products/core/launcher/src/child_handle.rs
use std::{
    process::Child,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub(crate) struct ChildHandle {
    pub(crate) child: Arc<Mutex<Child>>,
}
