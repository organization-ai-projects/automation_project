//! Common Parsing Library
// projects/libraries/common_parsing/src/lib.rs

use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
pub struct CursorPosition {
    pos: usize,
    line: usize,
    column: usize,
}

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

/// Parses unified diff (patch) text and returns unique touched path *tokens* as Strings.
/// This does NOT compute diffs, it only parses diff text.
pub fn parse_unified_diff_touched_path_strings(unified_diff: &str) -> Vec<String> {
    let mut seen = HashSet::<String>::new();
    let mut out = Vec::<String>::new();

    for p in unified_diff
        .lines()
        .filter_map(|line| line.strip_prefix("+++ b/"))
        .map(str::trim)
        .filter(|p| !p.is_empty() && *p != "/dev/null")
    {
        // preserve first-seen order
        if seen.insert(p.to_string()) {
            out.push(p.to_string());
        }
    }

    out
}

/// Parses unified diff (patch) text and returns touched paths as PathBuf.
/// Note: returned paths are NOT validated against the filesystem.
pub fn parse_unified_diff_touched_paths(unified_diff: &str) -> Vec<PathBuf> {
    parse_unified_diff_touched_path_strings(unified_diff)
        .into_iter()
        .map(PathBuf::from)
        .collect()
}

/// Parses a date string in the format `YYYY-MM-DD`. Returns the input if valid.
/// (Lightweight validation; does not validate month/day combinations like Feb 30.)
pub fn parse_date(date_str: &str) -> Option<String> {
    if date_str.len() != 10 {
        return None;
    }
    if date_str.as_bytes().get(4) != Some(&b'-') || date_str.as_bytes().get(7) != Some(&b'-') {
        return None;
    }

    let year = &date_str[0..4];
    let month = &date_str[5..7];
    let day = &date_str[8..10];

    let y_ok = year.parse::<u32>().is_ok();
    let m_ok = month
        .parse::<u32>()
        .ok()
        .is_some_and(|m| (1..=12).contains(&m));
    let d_ok = day
        .parse::<u32>()
        .ok()
        .is_some_and(|d| (1..=31).contains(&d));

    if y_ok && m_ok && d_ok {
        Some(date_str.to_string())
    } else {
        None
    }
}
