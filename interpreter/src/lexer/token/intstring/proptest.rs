use crate::lexer::*;

use proptest::prelude::*;

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
                                if ch == '{' || ch == '}' || ch == '\\' {
                                    "_".to_string()
                                } else if ch == sch {
                                    format!("\\{sch}")
                                } else {
                                    ch.to_string()
                                }
                            })
                            .collect::<String>(),
                    )
                })
                .boxed()
        } else {
            proptest::collection::vec(
                gens::rnd_kind(vec![
                    KindId::LeftBrace,
                    KindId::RightBrace,
                    KindId::CR,
                    KindId::LF,
                    KindId::CRLF,
                    KindId::Backtick,
                    KindId::SingleQuote,
                    KindId::DoubleQuote,
                    KindId::InterpolatedString, // remove from here
                    KindId::Number,
                    KindId::Whitespace,
                ]),
                1..5,
            )
            .prop_map(|mut knds| {
                knds.insert(0, vec![Kind::LeftBrace]);
                knds.push(vec![Kind::RightBrace]);
                let mut tokens: Vec<Token> = knds
                    .into_iter()
                    .flat_map(|knd| {
                        knd.into_iter()
                            .map(|knd| Token::by_pos(knd, 0, 0))
                            .collect::<Vec<Token>>()
                    })
                    .flat_map(|tk| {
                        if matches!(tk.id(), KindId::Comment | KindId::Meta) {
                            vec![tk]
                        } else {
                            vec![tk, Token::by_pos(Kind::Whitespace(String::from(" ")), 0, 0)]
                        }
                    })
                    .collect();
                if if let Some(tk) = tokens.last() {
                    tk.id() == KindId::Whitespace
                } else {
                    false
                } {
                    tokens.remove(tokens.len() - 1);
                }
                StringPart::Expression(tokens)
            })
            .boxed()
        }
    }
}
