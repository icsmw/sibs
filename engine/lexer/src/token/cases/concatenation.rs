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
        let from = lx.pos;
        let Some((literal, ch)) = lx.read_until(&[stop_char, left_brace]) else {
            return Err(E::NoClosingSymbol(stop_char));
        };
        if !literal.is_empty() {
            tokens.push(Token::by_pos(
                Kind::Literal(literal),
                &lx.uuid,
                from,
                lx.pos,
            ));
        }
        lx.advance();
        if ch == stop_char {
            tokens.push(Token::by_pos(target, &lx.uuid, lx.pos - 1, lx.pos));
            break;
        }
        let offset = lx.pos;
        let content = until_groupend(lx, left_brace, right_brace);
        let mut inner = Lexer::new(&content, 0);
        tokens.push(Token::by_pos(Kind::LeftBrace, &lx.uuid, offset - 1, offset));
        tokens.extend(
            inner
                .read()?
                .tokens
                .into_iter()
                .filter(|tk| !matches!(tk.id(), KindId::BOF | KindId::EOF))
                .map(|mut tk| {
                    tk.offset(offset);
                    tk
                })
                .collect::<Vec<Token>>(),
        );
        tokens.push(Token::by_pos(
            Kind::RightBrace,
            &lx.uuid,
            offset + content.len(),
            offset + content.len() + 1,
        ));
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
