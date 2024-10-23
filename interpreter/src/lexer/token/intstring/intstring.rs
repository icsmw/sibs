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
        let closed = loop {
            lx.advance();
            let Some(nch) = lx.char() else {
                break false;
            };
            if nch == '{' && !serialized {
                lx.advance();
                parts.push(StringPart::Literal(collected.clone()));
                collected.clear();
                let mut tokens: Vec<Token> = Vec::new();
                let mut prev = None;
                while let Some(tk) = Token::read(lx, prev)? {
                    prev = Some(tk.id());
                    tokens.push(tk);
                    if let Some('}') = lx.char() {
                        break;
                    }
                }
                if let Some('}') = lx.char() {
                    parts.push(StringPart::Expression(tokens));
                } else {
                    println!("{tokens:?}");
                    println!(">>>>>>>>>>>>>>>>>>>000");
                    return Err(E::NoClosingSymbol('}'));
                }
            } else if nch == ch {
                if !collected.is_empty() {
                    parts.push(StringPart::Literal(collected.clone()));
                }
                lx.advance();
                break true;
            } else {
                serialized = ch == '\\';
                collected.push(nch);
            }
        };
        if !closed {
            println!(">>>>>>>>>>>>>>>>>>>");
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
                Self::Expression(tokens) => format!(
                    "{{{}}}",
                    tokens
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                ),
            }
        )
    }
}
