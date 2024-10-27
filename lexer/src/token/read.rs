use crate::*;

/// Trait for reading tokens from a lexer.
///
/// This trait defines methods for reading tokens from a lexer,
/// both general (`read`) and specific (`try_read`) based on `KindId`.
pub trait Read {
    /// Reads the next token from the lexer.
    ///
    /// # Arguments
    ///
    /// * `lx` - A mutable reference to the lexer.
    /// * `tks` - A reference to the current list of tokens.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Token))` if a token was successfully read.
    /// * `Ok(None)` if no token could be read.
    /// * `Err(E)` if an error occurred.
    fn read(lx: &mut Lexer, tks: &Tokens) -> Result<Option<Token>, E>;

    /// Tries to read a token of a specific kind from the lexer.
    ///
    /// # Arguments
    ///
    /// * `lx` - A mutable reference to the lexer.
    /// * `id` - The kind identifier of the token to read.
    /// * `tks` - A reference to the current list of tokens.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Token))` if a token of the specified kind was successfully read.
    /// * `Ok(None)` if no token of the specified kind could be read.
    /// * `Err(E)` if an error occurred.
    fn try_read(lx: &mut Lexer, id: KindId, tks: &Tokens) -> Result<Option<Token>, E>;
}

impl Read for Token {
    fn read(lx: &mut Lexer, tks: &Tokens) -> Result<Option<Token>, E> {
        fn select(
            results: &mut Vec<(usize, Token, KindId)>,
            lx: &mut Lexer,
        ) -> Result<Option<Token>, E> {
            if let Some((n, (pos, tk, id))) =
                results.iter().enumerate().max_by_key(|(_, (a, ..))| a)
            {
                let conflicted = results
                    .iter()
                    .filter(|(p, _, oid)| p == pos && oid != id)
                    .cloned()
                    .collect::<Vec<(usize, Token, KindId)>>();

                if conflicted.is_empty() {
                    lx.pos = *pos;
                    Ok(Some(results.remove(n).1))
                } else if let (Some((_, c_tk, c_id)), true) =
                    (conflicted.first(), conflicted.len() == 1)
                {
                    lx.pos = *pos;
                    if &id.resolve_conflict(c_id) == id {
                        Ok(Some(tk.clone()))
                    } else {
                        Ok(Some(c_tk.clone()))
                    }
                } else {
                    Err(E::TokensAreInConflict(
                        results
                            .iter()
                            .filter(|(p, ..)| p == pos)
                            .map(|(.., id)| id.to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                    ))
                }
            } else {
                Ok(None)
            }
        }

        let drop = lx.pin();
        let next_ident = lx.read_identifier();
        drop(lx);

        let all: std::vec::IntoIter<KindId> = KindId::get_iter();
        let interested = all
            .filter(|el| el.interest_in_identifier(&next_ident))
            .collect::<Vec<KindId>>();

        let mut results = Vec::new();
        for id in interested.iter() {
            let drop = lx.pin();
            if let Some(tk) = Token::try_read(lx, id.clone(), tks)? {
                results.push((lx.pos, tk, id.clone()));
            }
            drop(lx);
        }
        if let Some(tk) = select(&mut results, lx)? {
            return Ok(Some(tk));
        }

        results.clear();
        let Some(next_ch) = lx.char() else {
            return Ok(None);
        };
        let all: std::vec::IntoIter<KindId> = KindId::get_iter();
        let interested = all
            .filter(|el| el.interest_in_char(&next_ch))
            .collect::<Vec<KindId>>();

        for id in interested.iter() {
            let drop = lx.pin();
            if let Some(tk) = Token::try_read(lx, id.clone(), tks)? {
                results.push((lx.pos, tk, id.clone()));
            }
            drop(lx);
        }
        if let Some(tk) = select(&mut results, lx)? {
            Ok(Some(tk))
        } else {
            Ok(None)
        }
    }

    fn try_read(lx: &mut Lexer, id: KindId, tks: &Tokens) -> Result<Option<Token>, E> {
        let from = lx.pos;
        match id {
            KindId::If
            | KindId::Else
            | KindId::While
            | KindId::Loop
            | KindId::For
            | KindId::Return
            | KindId::Let
            | KindId::True
            | KindId::False => Ok(if lx.read_identifier() == id.to_string() {
                Some(Token::by_pos(id.try_into()?, from, lx.pos))
            } else {
                None
            }),
            KindId::Whitespace => {
                let ws = lx.read_whitespace();
                Ok(if ws.is_empty() {
                    None
                } else {
                    Some(Token::by_pos(Kind::Whitespace(ws), from, lx.pos))
                })
            }
            KindId::Identifier => {
                let ident = lx.read_identifier();
                if let Some(ch) = ident.chars().next() {
                    if ch.is_numeric() {
                        return Ok(None);
                    }
                }
                Ok(if ident.is_empty() {
                    None
                } else {
                    Some(Token::by_pos(Kind::Identifier(ident), from, lx.pos))
                })
            }
            KindId::Number => {
                while let Some(c) = lx.char() {
                    if c.is_ascii_digit() || c == '.' {
                        lx.advance();
                    } else {
                        break;
                    }
                }
                let to = lx.pos;
                let snum = &lx.input[from..to];
                match snum.parse::<f64>() {
                    Ok(num) => Ok(Some(Token::by_pos(Kind::Number(num), from, to))),
                    Err(_) => Err(E::InvalidNumber),
                }
            }
            KindId::String => {
                if let Some('"') = lx.char() {
                    lx.advance();
                    if let Some(str) = lx.read_until('"') {
                        lx.advance();
                        Ok(Some(Token::by_pos(Kind::String(str), from, lx.pos)))
                    } else {
                        Err(E::NoClosingSymbol('\"'))
                    }
                } else {
                    Ok(None)
                }
            }
            KindId::Command => Ok(StringPart::try_read(lx, KindId::Backtick)?
                .map(|kind| Token::by_pos(kind, from, lx.pos))),
            KindId::InterpolatedString => Ok(StringPart::try_read(lx, KindId::SingleQuote)?
                .map(|kind| Token::by_pos(kind, from, lx.pos))),
            KindId::Comment => {
                if !tks.is_nl() {
                    return Ok(None);
                }
                if id.to_string() == lx.read_nth(2) {
                    let drop = lx.pin();
                    Ok(if let Some(content) = lx.read_until('\n') {
                        Some(Token::by_pos(Kind::Comment(content), from, lx.pos))
                    } else {
                        drop(lx);
                        Some(Token::by_pos(Kind::Comment(lx.read_to_end()), from, lx.pos))
                    })
                } else {
                    Ok(None)
                }
            }
            KindId::Meta => {
                if !tks.is_nl() {
                    return Ok(None);
                }
                if id.to_string() == lx.read_nth(3) {
                    let drop = lx.pin();
                    Ok(if let Some(content) = lx.read_until('\n') {
                        Some(Token::by_pos(Kind::Meta(content), from, lx.pos))
                    } else {
                        drop(lx);
                        Some(Token::by_pos(Kind::Meta(lx.read_to_end()), from, lx.pos))
                    })
                } else {
                    Ok(None)
                }
            }
            KindId::Question
            | KindId::SingleQuote
            | KindId::DoubleQuote
            | KindId::Tilde
            | KindId::Backtick
            | KindId::Dollar
            | KindId::At
            | KindId::Pound
            | KindId::Plus
            | KindId::Minus
            | KindId::Star
            | KindId::Slash
            | KindId::Percent
            | KindId::Equals
            | KindId::EqualEqual
            | KindId::BangEqual
            | KindId::Less
            | KindId::LessEqual
            | KindId::Greater
            | KindId::GreaterEqual
            | KindId::And
            | KindId::Or
            | KindId::VerticalBar
            | KindId::Bang
            | KindId::PlusEqual
            | KindId::MinusEqual
            | KindId::StarEqual
            | KindId::SlashEqual
            | KindId::LeftParen
            | KindId::RightParen
            | KindId::LeftBrace
            | KindId::RightBrace
            | KindId::LeftBracket
            | KindId::RightBracket
            | KindId::Comma
            | KindId::Colon
            | KindId::Dot
            | KindId::DotDot
            | KindId::Semicolon
            | KindId::Arrow
            | KindId::DoubleArrow
            | KindId::LF
            | KindId::CR
            | KindId::CRLF => Ok(if id.to_string() == lx.read_nth(id.length()?) {
                Some(Token::by_pos(id.try_into()?, from, lx.pos))
            } else {
                None
            }),
            KindId::EOF | KindId::BOF => Err(E::AttemptToReadEOForBOF),
        }
    }
}
