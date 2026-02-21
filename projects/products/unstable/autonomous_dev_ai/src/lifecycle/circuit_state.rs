// projects/products/unstable/autonomous_dev_ai/src/lifecycle/circuit_state.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}
