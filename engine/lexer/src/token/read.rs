use crate::*;

/// Trait for reading tokens from a lexer.
///
/// This trait defines methods for reading tokens from a lexer,
/// both general (`read`) and specific (`try_read`) based on `KindId`.
pub(crate) trait Read {
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
            results: &mut Vec<(TextPosition, Token, KindId)>,
            lx: &mut Lexer,
        ) -> Result<Option<Token>, E> {
            if let Some((n, (pos, tk, id))) =
                results.iter().enumerate().max_by_key(|(_, (a, ..))| a.abs)
            {
                let conflicted = results
                    .iter()
                    .filter(|(p, _, oid)| p == pos && oid != id)
                    .cloned()
                    .collect::<Vec<(TextPosition, Token, KindId)>>();

                if conflicted.is_empty() {
                    lx.set_pos(*pos);
                    Ok(Some(results.remove(n).1))
                } else if let (Some((_, c_tk, c_id)), true) =
                    (conflicted.first(), conflicted.len() == 1)
                {
                    lx.set_pos(*pos);
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

        let interested = KindId::as_vec()
            .into_iter()
            .filter(|el| el.interest_in_identifier(&next_ident))
            .collect::<Vec<KindId>>();

        let mut results = Vec::new();
        for id in interested.iter() {
            let drop = lx.pin();
            if let Some(tk) = Token::try_read(lx, id.clone(), tks)? {
                results.push((lx.current_pos(), tk, id.clone()));
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
        let interested = KindId::as_vec()
            .into_iter()
            .filter(|el| el.interest_in_char(&next_ch))
            .collect::<Vec<KindId>>();

        for id in interested.iter() {
            let drop = lx.pin();
            if let Some(tk) = Token::try_read(lx, id.clone(), tks)? {
                results.push((lx.current_pos(), tk, id.clone()));
            }
            drop(lx);
        }
        select(&mut results, lx)
    }

    fn try_read(lx: &mut Lexer, id: KindId, tks: &Tokens) -> Result<Option<Token>, E> {
        let from = lx.current_pos();
        match id {
            KindId::Keyword => Ok(
                if let Ok(kw) = TryInto::<Keyword>::try_into(lx.read_identifier()) {
                    Some(Token::by_pos(
                        Kind::Keyword(kw),
                        &lx.uuid,
                        from,
                        lx.current_pos(),
                    ))
                } else {
                    None
                },
            ),
            KindId::Whitespace => {
                let ws = lx.read_whitespace();
                Ok(if ws.is_empty() {
                    None
                } else {
                    Some(Token::by_pos(
                        Kind::Whitespace(ws),
                        &lx.uuid,
                        from,
                        lx.current_pos(),
                    ))
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
                    Some(Token::by_pos(
                        Kind::Identifier(ident),
                        &lx.uuid,
                        from,
                        lx.current_pos(),
                    ))
                })
            }
            KindId::Number => {
                let mut restore = lx.pin();
                let mut dot = false;
                while let Some(c) = lx.char() {
                    if !dot {
                        restore = lx.pin();
                    }
                    if c.is_ascii_digit() || c == '.' {
                        if c == '.' && lx.is_next('.') {
                            break;
                        }
                        dot = c == '.';
                        lx.advance();
                    } else {
                        break;
                    }
                }
                if lx.input[from.abs..lx.pos].ends_with('.') {
                    restore(lx);
                };
                let to = lx.current_pos();
                let snum = &lx.input[from.abs..to.abs];
                match snum.parse::<f64>() {
                    Ok(num) => Ok(Some(Token::by_pos(Kind::Number(num), &lx.uuid, from, to))),
                    Err(_) => Err(E::InvalidNumber),
                }
            }
            KindId::String => {
                if let Some('"') = lx.char() {
                    lx.advance();
                    if let Some((str, ..)) = lx.read_until(&['"']) {
                        lx.advance();
                        Ok(Some(Token::by_pos(
                            Kind::String(str),
                            &lx.uuid,
                            from,
                            lx.current_pos(),
                        )))
                    } else {
                        Err(E::NoClosingSymbol('\"'))
                    }
                } else {
                    Ok(None)
                }
            }
            KindId::Comment => {
                if !tks.is_nl() {
                    return Ok(None);
                }
                if id.as_str() == lx.read_nth(2) {
                    let drop = lx.pin();
                    Ok(if let Some((content, ..)) = lx.read_until(&['\n']) {
                        Some(Token::by_pos(
                            Kind::Comment(content),
                            &lx.uuid,
                            from,
                            lx.current_pos(),
                        ))
                    } else {
                        drop(lx);
                        Some(Token::by_pos(
                            Kind::Comment(lx.read_to_end()),
                            &lx.uuid,
                            from,
                            lx.current_pos(),
                        ))
                    })
                } else {
                    Ok(None)
                }
            }
            KindId::Meta => {
                if !tks.is_nl() {
                    return Ok(None);
                }
                if id.as_str() == lx.read_nth(3) {
                    let drop = lx.pin();
                    Ok(if let Some((content, ..)) = lx.read_until(&['\n']) {
                        Some(Token::by_pos(
                            Kind::Meta(content),
                            &lx.uuid,
                            from,
                            lx.current_pos(),
                        ))
                    } else {
                        drop(lx);
                        Some(Token::by_pos(
                            Kind::Meta(lx.read_to_end()),
                            &lx.uuid,
                            from,
                            lx.current_pos(),
                        ))
                    })
                } else {
                    Ok(None)
                }
            }
            KindId::Question
            | KindId::SingleQuote
            | KindId::DoubleQuote
            | KindId::Backslash
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
            | KindId::CRLF => Ok(if id.as_str() == lx.read_nth(id.length()?) {
                Some(Token::by_pos(
                    id.try_into()?,
                    &lx.uuid,
                    from,
                    lx.current_pos(),
                ))
            } else {
                None
            }),
            KindId::Literal => Err(E::AttemptToReadLiteral),
            KindId::EOF | KindId::BOF => Err(E::AttemptToReadEOForBOF),
        }
    }
}
