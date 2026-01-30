// projects/libraries/protocol_macros/src/display_mode.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DisplayMode {
    Display, // Uses {} formatting
    Debug,   // Uses {:?} formatting
}
