use crate::lexer::*;
use crate::tests::*;

use proptest::prelude::*;

fn get_rnd_kind(exceptions: Vec<KindId>) -> BoxedStrategy<Vec<Kind>> {
    prop_oneof![
        Just(KindId::If),
        Just(KindId::Else),
        Just(KindId::While),
        Just(KindId::Loop),
        Just(KindId::For),
        Just(KindId::Return),
        Just(KindId::Let),
        Just(KindId::True),
        Just(KindId::False),
        Just(KindId::Question),
        Just(KindId::Dollar),
        Just(KindId::At),
        Just(KindId::Pound),
        Just(KindId::Plus),
        Just(KindId::Minus),
        Just(KindId::Star),
        Just(KindId::Slash),
        Just(KindId::Percent),
        Just(KindId::Equals),
        Just(KindId::EqualEqual),
        Just(KindId::BangEqual),
        Just(KindId::Less),
        Just(KindId::LessEqual),
        Just(KindId::Greater),
        Just(KindId::GreaterEqual),
        Just(KindId::And),
        Just(KindId::Or),
        Just(KindId::VerticalBar),
        Just(KindId::Bang),
        Just(KindId::PlusEqual),
        Just(KindId::MinusEqual),
        Just(KindId::StarEqual),
        Just(KindId::SlashEqual),
        Just(KindId::LeftParen),
        Just(KindId::RightParen),
        Just(KindId::LeftBrace),
        Just(KindId::RightBrace),
        Just(KindId::LeftBracket),
        Just(KindId::RightBracket),
        Just(KindId::Comma),
        Just(KindId::Colon),
        Just(KindId::Dot),
        Just(KindId::DotDot),
        Just(KindId::Semicolon),
        Just(KindId::Arrow),
        Just(KindId::DoubleArrow),
        Just(KindId::Identifier),
        Just(KindId::Number),
        Just(KindId::String),
        Just(KindId::InterpolatedString),
        Just(KindId::Command),
        Just(KindId::Comment),
        Just(KindId::Meta),
    ]
    .prop_filter("Exception", move |id| {
        !exceptions.contains(id) && id != &KindId::EOF
    })
    .prop_flat_map(get_kind)
    .boxed()
}

fn get_kind(id: KindId) -> Vec<BoxedStrategy<Kind>> {
    match id {
        KindId::If
        | KindId::Else
        | KindId::While
        | KindId::Loop
        | KindId::For
        | KindId::Return
        | KindId::Let
        | KindId::True
        | KindId::False
        | KindId::Question
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
        | KindId::BOF
        | KindId::EOF
        | KindId::LF
        | KindId::CR
        | KindId::CRLF => vec![Just(id.try_into().expect("Fail convert KindId to Kind")).boxed()],
        KindId::Identifier => vec!["[a-z][a-z0-9]*"
            .prop_map(String::from)
            .prop_map(Kind::Identifier)
            .boxed()],
        KindId::Number => vec![proptest::num::f64::NORMAL
            .prop_filter("Finite f64", |x| x.is_finite())
            .prop_map(Kind::Number)
            .boxed()],
        KindId::String => vec![proptest::collection::vec(any::<char>(), 0..200)
            .prop_map(|chars| {
                Kind::String(
                    chars
                        .into_iter()
                        .map(|ch| {
                            if ch == '"' {
                                "\\\"".to_string()
                            } else if ch == '\\' {
                                "_".to_string()
                            } else {
                                ch.to_string()
                            }
                        })
                        .collect::<String>(),
                )
            })
            .boxed()],
        KindId::InterpolatedString => vec![(1..10, prop_oneof![Just(0u8), Just(1u8)])
            .prop_flat_map(|(count, first)| {
                let mut parts: Vec<BoxedStrategy<StringPart>> = Vec::new();
                let mut variant = first;
                for _ in 0..count {
                    parts.push(StringPart::arbitrary_with((variant, '\'')));
                    if variant == 0 {
                        variant = 1;
                    } else {
                        variant = 0;
                    }
                }
                parts.prop_map(Kind::InterpolatedString).boxed()
            })
            .boxed()],
        KindId::Command => vec![(1..10, prop_oneof![Just(0u8), Just(1u8)])
            .prop_flat_map(|(count, first)| {
                let mut parts: Vec<BoxedStrategy<StringPart>> = Vec::new();
                let mut variant = first;
                for _ in 0..count {
                    parts.push(StringPart::arbitrary_with((variant, '`')));
                    if variant == 0 {
                        variant = 1;
                    } else {
                        variant = 0;
                    }
                }
                parts.prop_map(Kind::Command).boxed()
            })
            .boxed()],
        KindId::Comment => vec![
            Just(Kind::LF).boxed(),
            proptest::collection::vec(any::<char>(), 0..200)
                .prop_map(|chars| {
                    Kind::Comment(format!(
                        "comment:{}",
                        chars
                            .into_iter()
                            .map(|ch| {
                                if ch == '\n' || ch == '\r' || ch == '\\' {
                                    "_".to_string()
                                } else {
                                    ch.to_string()
                                }
                            })
                            .collect::<String>()
                    ))
                })
                .boxed(),
            Just(Kind::LF).boxed(),
        ],
        KindId::Meta => vec![
            Just(Kind::LF).boxed(),
            proptest::collection::vec(any::<char>(), 0..200)
                .prop_map(|chars| {
                    Kind::Meta(format!(
                        "meta:{}",
                        chars
                            .into_iter()
                            .map(|ch| {
                                if ch == '\n' || ch == '\r' || ch == '\\' {
                                    "_".to_string()
                                } else {
                                    ch.to_string()
                                }
                            })
                            .collect::<String>()
                    ))
                })
                .boxed(),
            Just(Kind::LF).boxed(),
        ],
    }
}

impl Arbitrary for StringPart {
    type Parameters = (u8, char);
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((variant, sch): Self::Parameters) -> Self::Strategy {
        if variant == 0 {
            (proptest::collection::vec(any::<char>(), 1..100), Just(sch))
                .prop_map(|(chars, sch)| {
                    StringPart::Literal(
                        chars
                            .into_iter()
                            .map(|ch| {
                                if ch == '\''
                                    || ch == '`'
                                    || ch == '"'
                                    || ch == '{'
                                    || ch == '}'
                                    || ch == '\\'
                                {
                                    "_".to_string()
                                } else {
                                    ch.to_string()
                                }
                                // if ch == sch {
                                //     format!("\\{sch}")
                                // } else if ch == '{' {
                                //     "\\{".to_string()
                                // } else if ch == '}' {
                                //     "\\}".to_string()
                                // } else if ch == '\\' {
                                //     "_".to_string()
                                // } else {
                                //     ch.to_string()
                                // }
                            })
                            .collect::<String>(),
                    )
                })
                .boxed()
        } else {
            proptest::collection::vec(
                get_rnd_kind(vec![
                    KindId::LeftBrace,
                    KindId::RightBrace,
                    KindId::CR,
                    KindId::LF,
                    KindId::CRLF,
                    KindId::Slash,
                ]),
                1..5,
            )
            .prop_map(|knd| {
                StringPart::Expression(
                    knd.into_iter()
                        .flat_map(|knd| {
                            knd.into_iter()
                                .map(|knd| Token::by_pos(knd, 0, 0))
                                .collect::<Vec<Token>>()
                        })
                        .collect::<Vec<Token>>(),
                )
            })
            .boxed()
        }
    }
}

fn run(kinds: Vec<Vec<Kind>>) {
    let kinds = kinds.into_iter().flatten().collect::<Vec<Kind>>();
    let mut pos: usize = 0;
    let mut origin = String::new();
    let mut generated = kinds
        .into_iter()
        .map(|knd| {
            let mut token = Token::by_pos(knd, pos, 0);
            origin.push_str(token.to_string().as_str());
            token.pos.to = if !origin.is_empty() {
                origin.len() - 1
            } else {
                0
            };
            pos = origin.len();
            token
        })
        .collect::<Vec<Token>>();
    generated.insert(0, Token::by_pos(Kind::BOF, 0, 0));
    let mut lx = Lexer::new(&origin, 0);
    let tokens = lx.read(true);
    match tokens {
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
            // for (n, tk) in tokens.iter().enumerate() {
            //     assert_eq!(lx.input[tk.pos.from..tk.pos.to], tk.to_string());
            // }
        }
        Err(err) => {
            // println!(">>>>>>>>>>>>>>>REST:{}", lx.rest());
            println!(">>>>>>>>>>>>>>>REST DEB:{:?}", lx.rest());
            // println!(">>>>>>>>>>>>>>>ORIGIN:{origin}",);
            println!(">>>>>>>>>>>>>>>ORIGIN DEB:{origin:?}");
            panic!("{err:?}");
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        max_shrink_iters: 50,
        ..ProptestConfig::with_cases(100)
    })]

    #[test]
    fn string(cases in proptest::collection::vec(get_kind(KindId::String), 100)) {
        run(cases);
    }

    #[test]
    fn comment(cases in proptest::collection::vec(get_kind(KindId::Comment), 100)) {
        run(cases);
    }

    #[test]
    fn meta(cases in proptest::collection::vec(get_kind(KindId::Meta), 100)) {
        run(cases);
    }

    #[test]
    fn command(cases in proptest::collection::vec(get_kind(KindId::Command), 100)) {
        run(cases);
    }

    #[test]
    fn tokens(kinds in proptest::collection::vec(get_rnd_kind(vec![]), 1..100)) {
        let kinds = kinds.into_iter().flatten().collect::<Vec<Kind>>();
        let mut pos: usize = 0;
        let mut origin = String::new();
        let tokens = kinds.into_iter().map(|knd| {
            let mut token = Token::by_pos(knd, pos, 0);
            origin.push_str(token.to_string().as_str());
            token.pos.to = if !origin.is_empty() { origin.len() - 1 } else {
                0
            };
            pos = origin.len();
            token
        }).collect::<Vec<Token>>();

        let mut lx = Lexer::new(&origin, 0);
        let tokens = lx.read(true);
        match tokens {
            Ok(tokens) => {
                println!("{tokens:?}");
                // let restored = tokens
                //     .iter()
                //     .map(|tk| tk.to_string())
                //     .collect::<Vec<String>>()
                //     .join("");
                // assert_eq!(
                //     restored,
                //     origin
                // );
                // for tk in tokens.iter() {
                //     assert_eq!(
                //         lx.input[tk.pos.from..tk.pos.to].replace("\n", ""),
                //         tk.to_string().replace("\n", "")
                //     );
                // }
            }
            Err(err) => {
                println!("REST:{}", lx.rest());
                panic!("{err:?}");
            }
        }
    }
}
