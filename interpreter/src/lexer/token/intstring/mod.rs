#[cfg(test)]
mod proptest;

use crate::lexer::*;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum StringPart {
    Literal(String),
    Expression(Vec<Token>),
}

impl StringPart {
    pub fn try_read(lx: &mut Lexer, ch: char) -> Result<Option<Kind>, E> {
        let Some(nch) = lx.char() else {
            return Ok(None);
        };
        if nch != ch {
            return Ok(None);
        }
        let mut collected = String::new();
        let mut serialized: bool = false;
        let mut parts: Vec<StringPart> = Vec::new();
        let mut skip = false;
        let closed = loop {
            if !skip {
                lx.advance();
            } else {
                skip = false;
            }
            let Some(nch) = lx.char() else {
                break false;
            };
            if nch == '{' && !serialized {
                parts.push(StringPart::Literal(collected.clone()));
                collected.clear();
                let mut tokens: Vec<Token> =
                    vec![Token::by_pos(Kind::LeftBrace, lx.pos, lx.pos + 1)];
                let mut prev = None;
                let mut closed = false;
                lx.advance();
                while let Some(tk) = Token::read(lx, prev)? {
                    prev = Some(tk.id());
                    let id = tk.id();
                    tokens.push(tk);
                    if matches!(id, KindId::RightBrace) {
                        closed = true;
                        skip = true;
                        break;
                    }
                }
                if closed {
                    parts.push(StringPart::Expression(tokens));
                } else {
                    return Err(E::NoClosingSymbol('}'));
                }
            } else if nch == ch && !serialized {
                if !collected.is_empty() {
                    parts.push(StringPart::Literal(collected.clone()));
                }
                lx.advance();
                break true;
            } else {
                serialized = nch == '\\';
                collected.push(nch);
            }
        };
        if !closed {
            return Err(E::NoClosingSymbol(ch));
        }

        Ok(Some(if ch == '`' {
            Kind::Command(parts)
        } else {
            Kind::InterpolatedString(parts)
        }))
    }
}

impl fmt::Display for StringPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Literal(s) => s.to_owned(),
                Self::Expression(tokens) => tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
            }
        )
    }
}
