// projects/products/core/launcher/src/child_handle.rs
use std::{
    process::Child,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct ChildHandle {
    pub child: Arc<Mutex<Child>>,
}
