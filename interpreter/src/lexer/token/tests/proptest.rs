use crate::lexer::*;

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        max_shrink_iters: 50,
        ..ProptestConfig::with_cases(100)
    })]

    #[test]
    fn string(cases in proptest::collection::vec(gens::kind(KindId::String), 100)) {
        runners::test_tokens_by_kinds(cases);
    }

    #[test]
    fn comment(cases in proptest::collection::vec(gens::kind(KindId::Comment), 100)) {
        runners::test_tokens_by_kinds(cases);
    }

    #[test]
    fn meta(cases in proptest::collection::vec(gens::kind(KindId::Meta), 100)) {
        runners::test_tokens_by_kinds(cases);
    }

    #[test]
    fn command(cases in proptest::collection::vec(gens::kind(KindId::Command), 100)) {
        runners::test_tokens_by_kinds(cases);
    }

    #[test]
    fn tokens(kinds in proptest::collection::vec(gens::rnd_kind(vec![]), 1..100)) {
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
