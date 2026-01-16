/// A source span (byte offsets).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    /// Creates a new span.
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Returns the length of the span.
    pub fn len(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    /// Returns true if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Returns true if this span contains the given offset.
    pub fn contains(&self, offset: u32) -> bool {
        offset >= self.start && offset < self.end
    }
}
