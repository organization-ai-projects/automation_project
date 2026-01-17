// projects/products/core/engine/src/runtime/types.rs
// Types used in the runtime module

pub type BackendId = String;

pub struct Route {
    pub path: String,
    pub handler: fn(),
}
