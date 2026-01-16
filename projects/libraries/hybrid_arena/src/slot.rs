#[derive(Debug)]
pub struct Slot<T> {
    pub generation: u32,
    pub value: Option<T>,
}
