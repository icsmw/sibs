use std::ops::Not;

use crate::lexer::*;

#[derive(Debug)]
pub struct Lexer<'a> {
    pub(crate) input: &'a str,
    pub(crate) pos: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str, offset: usize) -> Self {
        Lexer { input, pos: offset }
    }

    pub(crate) fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    pub(crate) fn read_nth(&mut self, count: usize) -> String {
        let start = self.pos;
        for _ in 0..count {
            if let Some(c) = self.char() {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    pub(crate) fn char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub(crate) fn rest(&self) -> String {
        self.input[self.pos..].to_string()
    }

    pub(crate) fn complited(&self) -> bool {
        self.input[self.pos..].is_empty()
    }

    pub(crate) fn advance(&mut self) {
        if let Some(ch) = self.char() {
            self.pos += ch.len_utf8();
        }
    }

    pub(crate) fn align(&mut self) {
        while let Some(ch) = self.char() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub(crate) fn read_until(&mut self, stop: char) -> Option<String> {
        let mut str: String = String::new();
        let mut serialized: bool = false;
        while let Some(ch) = self.char() {
            if ch == stop && !serialized {
                return Some(str);
            }
            serialized = ch == '\\';
            str.push(ch);
            self.advance();
        }
        None
    }

    pub(crate) fn to_end(&mut self) -> String {
        let rest = self.rest();
        self.pos = self.input.len();
        rest
    }

    pub(crate) fn pin(&mut self) -> impl Fn(&mut Lexer) -> usize {
        let pos = self.pos;
        move |lexer: &mut Lexer| {
            let to_restore = lexer.pos;
            lexer.pos = pos;
            to_restore
        }
    }

    pub fn read(&mut self, new_file: bool) -> Result<Vec<Token>, E> {
        let mut tokens: Vec<Token> = if new_file {
            vec![Token::by_pos(Kind::BOF, 0, 0)]
        } else {
            Vec::new()
        };

        let mut prev = new_file.then_some(KindId::BOF);
        while let Some(tk) = Token::read(self, prev)? {
            prev = Some(tk.id());
            tokens.push(tk);
        }
        if !self.complited() {
            Err(E::FailRecognizeContent(self.pos))
        } else {
            Ok(tokens)
        }
    }
}
