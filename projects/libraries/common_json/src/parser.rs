// projects/libraries/common_json/src/parser.rs
use crate::Json;
use crate::json_error::{JsonError, JsonErrorCode, JsonResult};
use crate::value::{JsonMap, JsonNumber};
use common_parsing::Cursor;
use std::io::Read;

pub fn parse_str(input: &str) -> JsonResult<Json> {
    let mut parser = Parser::new(input);
    let value = parser.parse_value()?;
    parser.skip_whitespace();
    if !parser.is_eof() {
        return Err(parser.error("trailing characters"));
    }
    Ok(value)
}

pub fn parse_bytes(bytes: &[u8]) -> JsonResult<Json> {
    let input = std::str::from_utf8(bytes)
        .map_err(|err| JsonError::new(JsonErrorCode::ParseError).context(err.to_string()))?;
    parse_str(input)
}

pub fn parse_reader<R: Read>(mut reader: R) -> JsonResult<Json> {
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .map_err(|err| JsonError::new(JsonErrorCode::ParseError).context(err.to_string()))?;
    parse_bytes(&buffer)
}

struct Parser<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    fn is_eof(&self) -> bool {
        self.cursor.is_eof()
    }

    fn error(&self, message: &str) -> JsonError {
        JsonError::new(JsonErrorCode::ParseError).context(format!(
            "{} at line {}, column {}",
            message,
            self.cursor.line(),
            self.cursor.column()
        ))
    }

    fn peek_char(&self) -> Option<char> {
        self.cursor.peek_char()
    }

    fn next_char(&mut self) -> Option<char> {
        self.cursor.next_char()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if matches!(ch, ' ' | '\n' | '\t' | '\r') {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn parse_value(&mut self) -> JsonResult<Json> {
        self.skip_whitespace();
        match self.peek_char() {
            Some('n') => self.parse_literal("null", Json::Null),
            Some('t') => self.parse_literal("true", Json::Bool(true)),
            Some('f') => self.parse_literal("false", Json::Bool(false)),
            Some('"') => self.parse_string().map(Json::String),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some('-') | Some('0'..='9') => self.parse_number(),
            Some(other) => Err(self.error(&format!("unexpected character '{}'", other))),
            None => Err(self.error("unexpected end of input")),
        }
    }

    fn parse_literal(&mut self, literal: &str, value: Json) -> JsonResult<Json> {
        if self.cursor.input()[self.cursor.pos()..].starts_with(literal) {
            for _ in 0..literal.len() {
                self.next_char();
            }
            Ok(value)
        } else {
            Err(self.error(&format!("expected '{}'", literal)))
        }
    }

    fn parse_string(&mut self) -> JsonResult<String> {
        if self.next_char() != Some('"') {
            return Err(self.error("expected '\"'"));
        }

        let mut output = String::new();
        loop {
            let ch = self
                .next_char()
                .ok_or_else(|| self.error("unterminated string"))?;
            match ch {
                '"' => break,
                '\\' => output.push(self.parse_escape_sequence()?),
                '\n' | '\r' => return Err(self.error("unterminated string")),
                ch if ch <= '\u{1F}' => return Err(self.error("invalid control character")),
                ch => output.push(ch),
            }
        }
        Ok(output)
    }

    fn parse_escape_sequence(&mut self) -> JsonResult<char> {
        let esc = self
            .next_char()
            .ok_or_else(|| self.error("incomplete escape"))?;
        match esc {
            '"' => Ok('"'),
            '\\' => Ok('\\'),
            '/' => Ok('/'),
            'b' => Ok('\u{0008}'),
            'f' => Ok('\u{000C}'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            'u' => self.parse_unicode_escape(),
            _ => Err(self.error("invalid escape sequence")),
        }
    }

    fn parse_unicode_escape(&mut self) -> JsonResult<char> {
        let first = self.read_hex_quad()?;
        if (0xD800..=0xDBFF).contains(&first) {
            let saved = self.cursor.position();

            if self.next_char() == Some('\\') && self.next_char() == Some('u') {
                let second = self.read_hex_quad()?;
                if (0xDC00..=0xDFFF).contains(&second) {
                    let combined =
                        0x10000 + (((first - 0xD800) as u32) << 10) + ((second - 0xDC00) as u32);
                    return std::char::from_u32(combined)
                        .ok_or_else(|| self.error("invalid unicode scalar"));
                }
            }

            self.cursor.restore(saved);
            return Err(self.error("invalid unicode surrogate pair"));
        }

        std::char::from_u32(first as u32).ok_or_else(|| self.error("invalid unicode scalar"))
    }

    fn read_hex_quad(&mut self) -> JsonResult<u16> {
        let mut value: u16 = 0;
        for _ in 0..4 {
            let ch = self
                .next_char()
                .ok_or_else(|| self.error("incomplete unicode escape"))?;
            let digit = ch
                .to_digit(16)
                .ok_or_else(|| self.error("invalid hex digit"))? as u16;
            value = (value << 4) | digit;
        }
        Ok(value)
    }

    fn parse_array(&mut self) -> JsonResult<Json> {
        self.next_char();
        let mut values = Vec::new();
        loop {
            self.skip_whitespace();
            if let Some(']') = self.peek_char() {
                self.next_char();
                break;
            }
            let value = self.parse_value()?;
            values.push(value);
            self.skip_whitespace();
            match self.peek_char() {
                Some(',') => {
                    self.next_char();
                }
                Some(']') => {
                    self.next_char();
                    break;
                }
                _ => return Err(self.error("expected ',' or ']'")),
            }
        }
        Ok(Json::Array(values))
    }

    fn parse_object(&mut self) -> JsonResult<Json> {
        self.next_char();
        let mut map = JsonMap::new();
        loop {
            self.skip_whitespace();
            if let Some('}') = self.peek_char() {
                self.next_char();
                break;
            }
            let key = self.parse_string()?;
            self.skip_whitespace();
            if self.next_char() != Some(':') {
                return Err(self.error("expected ':'"));
            }
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_whitespace();
            match self.peek_char() {
                Some(',') => {
                    self.next_char();
                }
                Some('}') => {
                    self.next_char();
                    break;
                }
                _ => return Err(self.error("expected ',' or '}'")),
            }
        }
        Ok(Json::Object(map))
    }

    fn parse_number(&mut self) -> JsonResult<Json> {
        let start = self.cursor.pos();

        if self.peek_char() == Some('-') {
            self.next_char();
        }

        match self.peek_char() {
            Some('0') => {
                self.next_char();
                if matches!(self.peek_char(), Some('0'..='9')) {
                    return Err(self.error("leading zeros are not allowed"));
                }
            }
            Some('1'..='9') => {
                while matches!(self.peek_char(), Some('0'..='9')) {
                    self.next_char();
                }
            }
            _ => return Err(self.error("invalid number")),
        }

        if self.peek_char() == Some('.') {
            self.next_char();
            if !matches!(self.peek_char(), Some('0'..='9')) {
                return Err(self.error("invalid fraction"));
            }
            while matches!(self.peek_char(), Some('0'..='9')) {
                self.next_char();
            }
        }

        if matches!(self.peek_char(), Some('e') | Some('E')) {
            self.next_char();
            if matches!(self.peek_char(), Some('+') | Some('-')) {
                self.next_char();
            }
            if !matches!(self.peek_char(), Some('0'..='9')) {
                return Err(self.error("invalid exponent"));
            }
            while matches!(self.peek_char(), Some('0'..='9')) {
                self.next_char();
            }
        }

        let slice = &self.cursor.input()[start..self.cursor.pos()];
        let value: f64 = slice.parse().map_err(|_| self.error("invalid number"))?;
        let number = JsonNumber::from_f64(value).ok_or_else(|| self.error("invalid number"))?;
        Ok(Json::Number(number))
    }
}
