// projects/libraries/common_parsing/src/cursor_position.rs
#[derive(Clone, Copy, Debug)]
pub struct CursorPosition {
    pub(crate) pos: usize,
    pub(crate) line: usize,
    pub(crate) column: usize,
}
