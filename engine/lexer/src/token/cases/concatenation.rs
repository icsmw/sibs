use crate::*;
fn until_groupend(lx: &mut Lexer, open_ch: char, close_ch: char) -> String {
    const ESCAPE: char = '\\';
    let mut content = String::new();
    let mut opened = 1;
    let mut escaped = false;
    while let Some(ch) = lx.char() {
        lx.advance();
        if ch == ESCAPE {
            escaped = true;
            content.push(ch);
            continue;
        }
        if (ch == close_ch || ch == open_ch) && escaped {
            escaped = false;
            content.push(ch);
            continue;
        }
        escaped = false;
        if ch == close_ch {
            opened -= 1;
        }
        if ch == open_ch {
            opened += 1;
        }
        if opened == 0 {
            break;
        }
        content.push(ch);
    }
    content
}

fn process(lx: &mut Lexer, token: Token, kind: KindId) -> Result<Option<Vec<Token>>, E> {
    let target = token.kind.clone();
    let mut tokens = vec![token];
    let stop_char: char = kind.try_into()?;
    let left_brace: char = KindId::LeftBrace.try_into()?;
    let right_brace: char = KindId::RightBrace.try_into()?;
    loop {
        let from = lx.current_pos();
        let Some((literal, ch)) = lx.read_until(&[stop_char, left_brace]) else {
            return Err(E::NoClosingSymbol(stop_char));
        };
        if !literal.is_empty() {
            tokens.push(Token::by_pos(
                Kind::Literal(literal),
                &lx.uuid,
                from,
                lx.current_pos(),
            ));
        }
        lx.advance();
        if ch == stop_char {
            tokens.push(Token::by_pos(
                target,
                &lx.uuid,
                lx.prev_pos(),
                lx.current_pos(),
            ));
            break;
        }
        let from = lx.current_pos();
        let brace_from = lx.prev_pos();
        let content = until_groupend(lx, left_brace, right_brace);
        let mut inner = lx.inherit(&content);
        tokens.push(Token::by_pos(Kind::LeftBrace, &lx.uuid, brace_from, from));
        tokens.extend(
            inner
                .read()?
                .tokens
                .into_iter()
                .filter(|tk| !matches!(tk.id(), KindId::BOF | KindId::EOF))
                .map(|mut tk| {
                    tk.offset(from.abs);
                    tk
                })
                .collect::<Vec<Token>>(),
        );
        let from = lx.prev_pos();
        let to = lx.current_pos();
        tokens.push(Token::by_pos(Kind::RightBrace, &lx.uuid, from, to));
    }
    Ok(Some(tokens))
}

pub fn check(lx: &mut Lexer, token: Token) -> Result<Option<Vec<Token>>, E> {
    if !matches!(token.id(), KindId::Backtick | KindId::SingleQuote) {
        return Ok(None);
    }
    let id = token.id();
    process(lx, token, id)
}
