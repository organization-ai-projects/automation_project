// projects/libraries/common_parsing/src/cursor.rs
use crate::cursor_position::CursorPosition;

pub struct Cursor<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    pub fn position(&self) -> CursorPosition {
        CursorPosition {
            pos: self.pos,
            line: self.line,
            column: self.column,
        }
    }

    pub fn restore(&mut self, position: CursorPosition) {
        self.pos = position.pos;
        self.line = position.line;
        self.column = position.column;
    }
}
