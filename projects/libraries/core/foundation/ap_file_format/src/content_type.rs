/// Supported content types for the AP file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ContentType {
    /// Raw binary data
    Binary = 0,
    /// Plain UTF-8 text
    PlainText = 1,
    /// JSON-encoded data
    Json = 2,
    /// RON-encoded data
    Ron = 3,
    /// Image pixel data (accompanied by an image sub-header)
    Image = 4,
    /// Markdown text
    Markdown = 5,
}

impl ContentType {
    /// Convert a `u16` tag to a `ContentType`.
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(Self::Binary),
            1 => Some(Self::PlainText),
            2 => Some(Self::Json),
            3 => Some(Self::Ron),
            4 => Some(Self::Image),
            5 => Some(Self::Markdown),
            _ => None,
        }
    }
}
