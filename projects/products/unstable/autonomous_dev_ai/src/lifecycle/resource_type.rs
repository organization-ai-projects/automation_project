// projects/products/unstable/autonomous_dev_ai/src/lifecycle/resource_type.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Iterations,
    Time,
    CpuTime,
    Memory,
    ToolExecutions,
}
