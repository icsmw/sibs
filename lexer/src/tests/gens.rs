use crate::*;

use proptest::prelude::*;

/// Generates a `BoxedStrategy` that produces vectors of `Kind` instances,
/// excluding specified exceptions.
///
/// This function is used in property-based testing to create random sequences
/// of `Kind` tokens while omitting any kinds listed in the `exceptions`.
///
/// # Arguments
///
/// * `exceptions` - A vector of `KindId` instances to exclude from the generated strategies.
///
/// # Returns
///
/// A `BoxedStrategy` yielding vectors of `Kind` instances.
pub fn rnd_kind(exceptions: Vec<KindId>) -> BoxedStrategy<Vec<Kind>> {
    prop_oneof![
        Just(KindId::If),
        Just(KindId::Else),
        Just(KindId::While),
        Just(KindId::Loop),
        Just(KindId::For),
        Just(KindId::Each),
        Just(KindId::Return),
        Just(KindId::Break),
        Just(KindId::Let),
        Just(KindId::True),
        Just(KindId::False),
        Just(KindId::Question),
        Just(KindId::SingleQuote),
        Just(KindId::DoubleQuote),
        Just(KindId::Tilde),
        Just(KindId::Backtick),
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
        Just(KindId::Whitespace),
        Just(KindId::Meta),
    ]
    .prop_filter("Exception", move |id| {
        !exceptions.contains(id) && id != &KindId::EOF
    })
    .prop_flat_map(kind)
    .boxed()
}

/// Generates strategies for creating `Kind` instances based on the given `KindId`.
///
/// This function maps a `KindId` to one or more `BoxedStrategy<Kind>` instances,
/// which are used in property-based testing to generate random `Kind` tokens.
///
/// # Arguments
///
/// * `id` - The `KindId` for which to generate strategies.
///
/// # Returns
///
/// A vector of `BoxedStrategy<Kind>` instances corresponding to the provided `KindId`.
pub fn kind(id: KindId) -> Vec<BoxedStrategy<Kind>> {
    match id {
        KindId::If
        | KindId::Else
        | KindId::While
        | KindId::Loop
        | KindId::For
        | KindId::Each
        | KindId::Return
        | KindId::Break
        | KindId::Let
        | KindId::True
        | KindId::False
        | KindId::Question
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
        | KindId::BOF
        | KindId::EOF
        | KindId::LF
        | KindId::CR
        | KindId::CRLF
        | KindId::Whitespace => {
            vec![Just(id.try_into().expect("Failed to convert KindId to Kind")).boxed()]
        }
        KindId::Identifier => vec!["[a-z][a-z0-9]*"
            .prop_map(String::from)
            .prop_filter("conflicts", |s| {
                if let Some(ch) = s.chars().next() {
                    if ch.is_numeric() {
                        return false;
                    }
                }
                ![
                    KindId::If.to_string(),
                    KindId::Let.to_string(),
                    KindId::Return.to_string(),
                    KindId::Break.to_string(),
                    KindId::While.to_string(),
                    KindId::Else.to_string(),
                    KindId::For.to_string(),
                    KindId::Each.to_string(),
                    KindId::Loop.to_string(),
                    KindId::True.to_string(),
                    KindId::False.to_string(),
                ]
                .contains(s)
            })
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
                let mut parts: Vec<BoxedStrategy<StringPart>> = vec![Just(StringPart::Open(
                    Token::by_pos(Kind::SingleQuote, &Uuid::new_v4(), 0, 0),
                ))
                .boxed()];
                let mut variant = first;
                for _ in 0..count {
                    parts.push(StringPart::arbitrary_with((
                        variant,
                        KindId::SingleQuote.try_into().expect("SingleQuote as char"),
                    )));
                    variant = if variant == 0 { 1 } else { 0 };
                }
                parts.push(
                    Just(StringPart::Close(Token::by_pos(
                        Kind::SingleQuote,
                        &Uuid::new_v4(),
                        0,
                        0,
                    )))
                    .boxed(),
                );
                parts.prop_map(Kind::InterpolatedString).boxed()
            })
            .boxed()],
        KindId::Command => vec![(1..10, prop_oneof![Just(0u8), Just(1u8)])
            .prop_flat_map(|(count, first)| {
                let mut parts: Vec<BoxedStrategy<StringPart>> = vec![Just(StringPart::Open(
                    Token::by_pos(Kind::Backtick, &Uuid::new_v4(), 0, 0),
                ))
                .boxed()];
                let mut variant = first;
                for _ in 0..count {
                    parts.push(StringPart::arbitrary_with((
                        variant,
                        KindId::Backtick.try_into().expect("Backtick as char"),
                    )));
                    variant = if variant == 0 { 1 } else { 0 };
                }
                parts.push(
                    Just(StringPart::Close(Token::by_pos(
                        Kind::Backtick,
                        &Uuid::new_v4(),
                        0,
                        0,
                    )))
                    .boxed(),
                );
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
