#[derive(Debug, Clone)]
pub enum Action {
    Started(String),
    Finished(i32),
    Failed(String),
}
