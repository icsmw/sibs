#[cfg(test)]
use crate::*;

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        max_shrink_iters: 50,
        ..ProptestConfig::with_cases(500)
    })]

    #[test]
    fn statement_if(cases in proptest::collection::vec(If::arbitrary(), 1)) {
        for case in cases.into_iter() {
            println!("{case}");
        }
    }

}
