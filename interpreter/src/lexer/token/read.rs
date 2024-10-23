use crate::lexer::*;

pub trait Read {
    fn read(lx: &mut Lexer, prev: Option<KindId>) -> Result<Option<Token>, E>;
    fn try_read(lx: &mut Lexer, id: KindId, prev: Option<&KindId>) -> Result<Option<Token>, E>;
}

impl Read for Token {
    fn read(lx: &mut Lexer, prev: Option<KindId>) -> Result<Option<Token>, E> {
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
        lx.align();
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
            if let Some(tk) = Token::try_read(lx, id.clone(), prev.as_ref())? {
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
            if let Some(tk) = Token::try_read(lx, id.clone(), prev.as_ref())? {
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

    fn try_read(lx: &mut Lexer, id: KindId, prev: Option<&KindId>) -> Result<Option<Token>, E> {
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
            KindId::Identifier => {
                let ident = lx.read_identifier();
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
            KindId::Command => {
                Ok(StringPart::try_read(lx, '`')?.map(|kind| Token::by_pos(kind, from, lx.pos)))
            }
            KindId::InterpolatedString => {
                Ok(StringPart::try_read(lx, '\'')?.map(|kind| Token::by_pos(kind, from, lx.pos)))
            }
            KindId::Comment => {
                let Some(KindId::LF | KindId::CR | KindId::CRLF | KindId::BOF) = prev else {
                    return Ok(None);
                };
                if id.to_string() == lx.read_nth(2) {
                    let drop = lx.pin();
                    Ok(if let Some(content) = lx.read_until('\n') {
                        Some(Token::by_pos(Kind::Comment(content), from, lx.pos))
                    } else {
                        drop(lx);
                        Some(Token::by_pos(Kind::Comment(lx.to_end()), from, lx.pos))
                    })
                } else {
                    Ok(None)
                }
            }
            KindId::Meta => {
                let Some(KindId::LF | KindId::CR | KindId::CRLF | KindId::BOF) = prev else {
                    return Ok(None);
                };
                if id.to_string() == lx.read_nth(3) {
                    let drop = lx.pin();
                    Ok(if let Some(content) = lx.read_until('\n') {
                        Some(Token::by_pos(Kind::Meta(content), from, lx.pos))
                    } else {
                        drop(lx);
                        Some(Token::by_pos(Kind::Meta(lx.to_end()), from, lx.pos))
                    })
                } else {
                    Ok(None)
                }
            }
            KindId::Question
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

#[test]
fn test() {
    let content = r#"import(./common.sibs);
import(./defaults.sibs);
/// Application 
#(app: ./app)
    hash ("./src", "./dist", :client, :shared, :wrapper) -> (:build("dev"));
    /// Building electron application 
    /// build dev >> build developing version
    /// build prod >> build production version
    @build($mode: dev | prod) {
        $dev = envvar(DEV_MODE);
        cd(./some_folder);
        // Comment
        // Comment yarn 툃.옻A%b󈀀RL򢫕'󴓇\u{202e}<%.I򑰒q~%򵺜򤆅𸮟󵏅=1�vlong comment long comment long comment long comment long comment long comment long comment long comment long comment long comment
        cd(..);
        "yarn 툃.옻A%b󈀀RL򢫕'󴓇<%.I򑰒q~%򵺜򤆅𸮟󵏅=1�v
XѨ`󠗐P򎨈�񶨆$,::Ⱥ:%<S/<
        Wú`$$P.'􋹇Svug run build"
        "\u{38d8a}\0v\u{101bb3}.\u{202e}\u{1948d}\u{1b}?@<\u{feff};\u{202e}?\u{e62d6}\u{5ca75}\u{bfd17}\u{36130}v\u{47903}Ⱥ럈'<{'/%\0\u{34e66}&吃??\u{49c68}h\u{e84b9}\t.l\n`H"
        remove(./node_modules/shared);
        remove(./node_modules/wrapper);
        remove(./dist/client);
        :shared:prod;
        copy(../shared, ./node_modules);
        :wrapper:prod;
        copy(../core/wrapper, ./node_modules);
        $mode == "dev" => :client:build("dev");
        $mode == "prod" => :client:build("prod");
        // Comment
        // Comment 1long comment  2long comment  3long comment  4long comment  5long comment  6long comment 7long comment  8long comment  9long comment  10long comment 11long comment  12long comment  13long comment  14long comment  15long comment  16long comment 17long comment  18long comment  19long comment  20long comment
        copy(../client/dist/client, ./dist);
        copy(./package.json, ./dist);
        `command_a{variable}command_b`
        `command_a{variable"string"variable'string_a{variable}string_b'}command_b`
        `command_a{variable}`
        `{variable}command_b`
        `¡{//=}`
        `some{ident
        // comment A
        identB
        //comment B
        identC
        }rest`
    };"#;
    let mut lx = Lexer::new(content, 0);
    let tokens = lx.read(true);
    match tokens {
        Ok(tokens) => {
            println!("{tokens:?}");
            let restored = tokens
                .iter()
                .map(|tk| tk.to_string())
                .collect::<Vec<String>>()
                .join("");
            assert_eq!(
                restored.replace(" ", "").replace("\n", ""),
                content.to_string().replace(" ", "").replace("\n", "")
            );
            for tk in tokens.iter() {
                assert_eq!(
                    lx.input[tk.pos.from..tk.pos.to].replace("\n", ""),
                    tk.to_string().replace("\n", "")
                );
            }
        }
        Err(err) => {
            println!("ERR: {err}");
            println!("REST:{}", lx.rest());
        }
    }
}
