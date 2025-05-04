use crate::*;
use proptest::prelude::*;

const MAX_DEEP: u8 = 5;

pub fn content(id: Kind, deep: u8) -> BoxedStrategy<Vec<Kind>> {
    (
        if deep < MAX_DEEP {
            content(id.clone(), deep + 1)
        } else {
            Just(Vec::new()).boxed()
        },
        proptest::collection::vec(gens::kind(KindId::Literal), 0..10),
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
                KindId::Whitespace,
                KindId::Equals,
                KindId::Literal,
                KindId::Comment,
                KindId::Meta,
                KindId::EOF,
                KindId::BOF,
            ]),
            0..10,
        ),
    )
        .prop_map(move |(mut nested, mut literals, injections)| {
            let mut output = vec![id.clone()];
            let inject = injections
                .clone()
                .into_iter()
                .flat_map(|knd| vec![knd, Kind::Semicolon])
                .collect::<Vec<Kind>>();
            loop {
                if literals.is_empty() {
                    break;
                }
                if !literals.is_empty() {
                    let literal = literals.remove(0);
                    if !literal.to_string().is_empty() {
                        output.push(literal);
                    }
                }
                if !nested.is_empty() {
                    output.push(Kind::LeftBrace);
                    output.extend(inject.clone());
                    output.extend(std::mem::take(&mut nested));
                    output.extend(inject.clone());
                    output.push(Kind::RightBrace);
                }
                output.push(Kind::LeftBrace);
                output.extend(inject.clone());
                output.push(Kind::RightBrace);
            }
            output.push(id.clone());
            output
        })
        .boxed()
}
