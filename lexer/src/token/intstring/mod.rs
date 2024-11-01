#[cfg(any(test, feature = "proptests"))]
mod proptests;

use crate::*;
use std::fmt;

/// Represents a part of an interpolated string or command in the source code.
///
/// The `StringPart` enum is used to represent the different components that can occur within
/// an interpolated string or command, such as the opening token, literals, expressions, and closing token.
#[derive(Debug, PartialEq, Clone)]
pub enum StringPart {
    /// The opening token of the string or command (e.g., `"`, `'`, or `` ` ``).
    Open(Token),
    /// A literal string segment.
    Literal(String),
    /// An expression enclosed within `{}`.
    Expression(Vec<Token>),
    /// The closing token of the string or command.
    Close(Token),
}

impl StringPart {
    /// Attempts to read a `StringPart` from the lexer.
    ///
    /// This method tries to parse an interpolated string or command starting with the specified `KindId`.
    ///
    /// # Arguments
    ///
    /// * `lx` - A mutable reference to the lexer.
    /// * `knd` - The kind identifier of the starting token (e.g., `KindId::SingleQuote` or `KindId::Backtick`).
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Kind))` if a string part was successfully read.
    /// * `Ok(None)` if no string part could be read.
    /// * `Err(E)` if an error occurred during parsing.
    pub fn try_read(lx: &mut Lexer, knd: KindId) -> Result<Option<Kind>, E> {
        let Some(nch) = lx.char() else {
            return Ok(None);
        };
        let stop_ch: char = knd.clone().try_into()?;
        if nch != stop_ch {
            return Ok(None);
        }
        let mut collected = String::new();
        let mut serialized: bool = false;
        let mut parts: Vec<StringPart> = vec![StringPart::Open(Token::by_pos(
            knd.clone().try_into()?,
            lx.pos,
            lx.pos + knd.length()?,
        ))];
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
                if !collected.is_empty() {
                    parts.push(StringPart::Literal(collected.clone()));
                    collected.clear();
                }
                let mut tokens =
                    Tokens::with(vec![Token::by_pos(Kind::LeftBrace, lx.pos, lx.pos + 1)]);
                let mut closed = false;
                lx.advance();
                while let Some(tk) = Token::read(lx, &tokens)? {
                    tokens.add(tk);
                    if let Some(KindId::RightBrace) = tokens.last().map(|tk| tk.id()) {
                        closed = true;
                        skip = true;
                        break;
                    }
                }
                if closed {
                    parts.push(StringPart::Expression(tokens.tokens));
                } else {
                    return Err(E::NoClosingSymbol('}'));
                }
            } else if nch == stop_ch && !serialized {
                if !collected.is_empty() {
                    parts.push(StringPart::Literal(collected.clone()));
                }
                break true;
            } else {
                serialized = nch == '\\';
                collected.push(nch);
            }
        };
        if !closed {
            return Err(E::NoClosingSymbol(stop_ch));
        }
        parts.push(StringPart::Close(Token::by_pos(
            knd.clone().try_into()?,
            lx.pos,
            lx.pos + knd.length()?,
        )));
        lx.advance();
        Ok(Some(if knd == KindId::Backtick {
            Kind::Command(parts)
        } else {
            Kind::InterpolatedString(parts)
        }))
    }
}

impl fmt::Display for StringPart {
    /// Formats the `StringPart` into its string representation.
    ///
    /// This implementation concatenates the string representation of the parts,
    /// which is useful for reconstructing the original string or for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Open(tk) => tk.to_string(),
                Self::Literal(s) => s.to_owned(),
                Self::Expression(tokens) => tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
                Self::Close(tk) => tk.to_string(),
            }
        )
    }
}
