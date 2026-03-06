// projects/products/unstable/universal_model_engine/backend/src/determinism/deterministic_order.rs
pub struct DeterministicOrder;

impl DeterministicOrder {
    pub fn order(mut values: Vec<String>) -> Vec<String> {
        values.sort();
        values
    }
}
