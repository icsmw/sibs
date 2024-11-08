#[cfg(test)]
use crate::*;

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        max_shrink_iters: 50,
        ..ProptestConfig::with_cases(500)
    })]

    /// Tests the lexer with random string tokens.
    #[test]
    fn string(cases in proptest::collection::vec(gens::kind(KindId::String), 100)) {
        runners::test_tokens_by_kinds(cases.into_iter().collect());
    }

    /// Tests the lexer with random comment tokens.
    #[test]
    fn comment(cases in proptest::collection::vec(gens::kind(KindId::Comment), 100)) {
        runners::test_tokens_by_kinds(cases.into_iter().flat_map(gens::add_bound_kinds).collect());
    }

    /// Tests the lexer with random meta tokens.
    #[test]
    fn meta(cases in proptest::collection::vec(gens::kind(KindId::Meta), 100)) {
        runners::test_tokens_by_kinds(cases.into_iter().flat_map(gens::add_bound_kinds).collect());
    }

    /// Tests the lexer with random command tokens.
    #[test]
    fn command(cases in proptest::collection::vec(gens::kind(KindId::Command), 100)) {
        runners::test_tokens_by_kinds(cases.into_iter().collect());
    }

    /// Tests the lexer with random interpolated string tokens.
    #[test]
    fn interpolated_string(cases in proptest::collection::vec(gens::kind(KindId::InterpolatedString), 100)) {
        runners::test_tokens_by_kinds(cases.into_iter().collect());
    }

    /// Tests the lexer with combinations of random tokens.
    #[test]
    fn combination(cases in proptest::collection::vec(gens::rnd_kind(vec![KindId::Whitespace, KindId::SingleQuote, KindId::DoubleQuote, KindId::Backtick]), 1..1000)) {
        let mut cases = cases.into_iter().flat_map(|knd| {
            if matches!(knd.id(), KindId::Comment | KindId::Meta) {
                gens::add_bound_kinds(knd)
            } else {
                [gens::add_bound_kinds(knd), vec![Kind::Whitespace(String::from(" "))]].concat()
            }
        }).collect::<Vec<Kind>>();
        if if let Some(knd) = cases.last() {
            knd.id() == KindId::Whitespace
        } else {
            false
        } {
            cases.remove(cases.len() - 1);
        }
        runners::test_tokens_by_kinds(cases);
    }
}
