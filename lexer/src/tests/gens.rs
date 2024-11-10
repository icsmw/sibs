use crate::*;

use proptest::prelude::*;

/// Generates a `BoxedStrategy` with random `Kind` instance,
///
/// # Returns
///
/// A `BoxedStrategy` with random`Kind` instance.
pub fn rnd_kind() -> BoxedStrategy<Kind> {
    prop::strategy::Union::new(KindId::as_vec().into_iter().map(kind)).boxed()
}

/// Generates a `BoxedStrategy` with random `Kind` instance, including specified Kind.
///
/// # Arguments
///
/// * `includes` - A vector of `KindId` instances to include from the generated strategies.
///
/// # Returns
///
/// A `BoxedStrategy` with random`Kind` instance.
pub fn rnd_kind_with(includes: Vec<KindId>) -> BoxedStrategy<Kind> {
    prop::strategy::Union::new(includes.into_iter().map(kind)).boxed()
}

/// Generates a `BoxedStrategy` with random `Kind` instance, excluding specified exceptions.
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
/// A `BoxedStrategy` with random`Kind` instance.
pub fn rnd_kind_without(exceptions: Vec<KindId>) -> BoxedStrategy<Kind> {
    let includes = KindId::as_vec()
        .into_iter()
        .filter(|id| !exceptions.contains(id))
        .collect::<Vec<KindId>>();
    prop::strategy::Union::new(includes.into_iter().map(kind)).boxed()
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
/// `BoxedStrategy<Kind>` instance corresponding to the provided `KindId`.
pub fn kind(id: KindId) -> BoxedStrategy<Kind> {
    // TODO: return only one Kind, not a vector
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
        | KindId::In
        | KindId::Join
        | KindId::OneOf
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
            Just(id.try_into().expect("Failed to convert KindId to Kind")).boxed()
        }
        KindId::Identifier => "[a-z][a-z0-9]*"
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
                    KindId::In.to_string(),
                    KindId::Join.to_string(),
                    KindId::OneOf.to_string(),
                ]
                .contains(s)
            })
            .prop_map(Kind::Identifier)
            .boxed(),
        KindId::Number => proptest::num::f64::NORMAL
            .prop_filter("Finite f64", |x| x.is_finite())
            .prop_map(Kind::Number)
            .boxed(),
        KindId::String => proptest::collection::vec(any::<char>(), 0..200)
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
            .boxed(),
        KindId::InterpolatedString => (1..10, prop_oneof![Just(0u8), Just(1u8)])
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
            .boxed(),
        KindId::Command => (1..10, prop_oneof![Just(0u8), Just(1u8)])
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
            .boxed(),
        KindId::Comment => proptest::collection::vec(any::<char>(), 0..200)
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
        KindId::Meta => proptest::collection::vec(any::<char>(), 0..200)
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
    }
}

/// Adds bound Kinds. For example Comment and Meta needs to be around LF
pub fn add_bound_kinds(knd: Kind) -> Vec<Kind> {
    match &knd {
        Kind::Comment(..) | Kind::Meta(..) => {
            vec![Kind::LF, knd, Kind::LF]
        }
        _ => vec![knd],
    }
}
