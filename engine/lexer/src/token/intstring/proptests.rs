use std::ops::RangeInclusive;

use crate::*;

use gens::gen_string;
use proptest::prelude::*;

impl Arbitrary for StringPart {
    /// Parameters used for generating arbitrary `StringPart`.
    type Parameters = (u8, char);

    /// Strategy used for generating arbitrary `StringPart`.
    type Strategy = BoxedStrategy<Self>;

    /// Generates an arbitrary `StringPart` based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `(variant, sch)` - A tuple where `variant` determines the variant of `StringPart` to generate,
    ///   and `sch` is the stop character used in the string.
    fn arbitrary_with((variant, sch): Self::Parameters) -> Self::Strategy {
        if variant == 0 {
            (gen_string(RangeInclusive::new(1, 100)), Just(sch))
                .prop_map(|(str, sch)| {
                    StringPart::Literal(Token::by_pos(
                        Kind::Literal(
                            str.chars()
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
                        ),
                        &Uuid::new_v4(),
                        0,
                        0,
                    ))
                })
                .boxed()
        } else {
            proptest::collection::vec(
                gens::rnd_kind_without(vec![
                    KindId::LeftBrace,
                    KindId::RightBrace,
                    KindId::CR,
                    KindId::LF,
                    KindId::CRLF,
                    KindId::Backtick,
                    KindId::SingleQuote,
                    KindId::DoubleQuote,
                    KindId::Number,
                    KindId::Whitespace,
                    KindId::Literal,
                    KindId::EOF,
                    KindId::BOF,
                ]),
                1..5,
            )
            .prop_map(|knds| {
                let mut knds: Vec<Kind> =
                    knds.into_iter().flat_map(gens::add_bound_kinds).collect();
                knds.insert(0, Kind::LeftBrace);
                knds.push(Kind::RightBrace);
                let mut tokens: Vec<Token> = knds
                    .into_iter()
                    .map(|knd| Token::by_pos(knd, &Uuid::new_v4(), 0, 0))
                    .flat_map(|tk| {
                        if matches!(tk.id(), KindId::Comment | KindId::Meta) {
                            vec![tk]
                        } else {
                            vec![
                                tk,
                                Token::by_pos(
                                    Kind::Whitespace(String::from(" ")),
                                    &Uuid::new_v4(),
                                    0,
                                    0,
                                ),
                            ]
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
