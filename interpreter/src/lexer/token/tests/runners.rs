use crate::lexer::*;

impl Token {
    pub fn set_pos(&mut self, from: usize) -> usize {
        self.pos.from = from;
        match &mut self.kind {
            Kind::If
            | Kind::Else
            | Kind::While
            | Kind::Loop
            | Kind::For
            | Kind::Return
            | Kind::Let
            | Kind::True
            | Kind::False
            | Kind::Question
            | Kind::Dollar
            | Kind::At
            | Kind::Pound
            | Kind::Plus
            | Kind::Minus
            | Kind::Star
            | Kind::Slash
            | Kind::Percent
            | Kind::Equals
            | Kind::EqualEqual
            | Kind::BangEqual
            | Kind::Less
            | Kind::LessEqual
            | Kind::Greater
            | Kind::GreaterEqual
            | Kind::And
            | Kind::Or
            | Kind::VerticalBar
            | Kind::Bang
            | Kind::PlusEqual
            | Kind::MinusEqual
            | Kind::StarEqual
            | Kind::SlashEqual
            | Kind::LeftParen
            | Kind::RightParen
            | Kind::LeftBrace
            | Kind::RightBrace
            | Kind::LeftBracket
            | Kind::RightBracket
            | Kind::Comma
            | Kind::Dot
            | Kind::DotDot
            | Kind::Semicolon
            | Kind::Colon
            | Kind::Arrow
            | Kind::DoubleArrow
            | Kind::LF
            | Kind::CR
            | Kind::CRLF
            | Kind::EOF
            | Kind::BOF => {
                self.pos.to = from + self.kind.id().length().expect("Fail to get element length");
            }
            Kind::Identifier(..)
            | Kind::String(..)
            | Kind::Comment(..)
            | Kind::Meta(..)
            | Kind::Number(..) => {
                self.pos.to = from + self.to_string().len();
            }
            Kind::InterpolatedString(parts) | Kind::Command(parts) => {
                let mut pos = from;
                self.pos.to = from;
                parts.iter_mut().for_each(|part| {
                    self.pos.to += part.to_string().len();
                    match part {
                        StringPart::Literal(s) => {
                            pos += s.len();
                        }
                        StringPart::Expression(tks) => {
                            tks.iter_mut().for_each(|tk| {
                                pos = tk.set_pos(pos);
                            });
                        }
                    }
                });
            }
        };
        self.pos.to
    }
}

fn kinds_into(knds: Vec<Kind>) -> (Vec<Token>, String) {
    let mut pos: usize = 0;
    let mut content = String::new();
    let tokens = knds
        .into_iter()
        .map(|knd| {
            let mut token = Token::by_pos(knd, pos, 0);
            content.push_str(token.to_string().as_str());
            token.set_pos(pos);
            pos = content.len();
            token
        })
        .collect::<Vec<Token>>();
    (tokens, content)
}

pub fn test_tokens_by_kinds(kinds: Vec<Vec<Kind>>) {
    let (mut generated, origin) = kinds_into(kinds.into_iter().flatten().collect::<Vec<Kind>>());
    generated.insert(0, Token::by_pos(Kind::BOF, 0, 0));
    let mut lx = Lexer::new(&origin, 0);
    match lx.read(true) {
        Ok(tokens) => {
            let restored = tokens
                .iter()
                .map(|tk| tk.to_string())
                .collect::<Vec<String>>()
                .join("");
            assert_eq!(restored, origin);
            for tk in tokens.iter() {
                assert_eq!(lx.input[tk.pos.from..tk.pos.to], tk.to_string());
            }
            assert_eq!(tokens.len(), generated.len());
            for (n, tk) in tokens.iter().enumerate() {
                println!("READED: {tk:?}");
                println!("GEN: {:?}", generated[n]);
                assert_eq!(tk, &generated[n]);
            }
        }
        Err(err) => {
            panic!("{err:?}");
        }
    }
}
