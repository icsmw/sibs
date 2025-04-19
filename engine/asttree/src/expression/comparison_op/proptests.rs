use crate::*;
use proptest::prelude::*;

impl Arbitrary for ComparisonOperator {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(ComparisonOperator::Less),
            Just(ComparisonOperator::LessEqual),
            Just(ComparisonOperator::Greater),
            Just(ComparisonOperator::GreaterEqual),
            Just(ComparisonOperator::EqualEqual),
            Just(ComparisonOperator::BangEqual),
        ]
        .boxed()
    }
}

impl Arbitrary for ComparisonOp {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        ComparisonOperator::arbitrary()
            .prop_map(|operator| {
                let token = Token::for_test(match &operator {
                    ComparisonOperator::Less => Kind::Less,
                    ComparisonOperator::LessEqual => Kind::LessEqual,
                    ComparisonOperator::Greater => Kind::Greater,
                    ComparisonOperator::GreaterEqual => Kind::GreaterEqual,
                    ComparisonOperator::EqualEqual => Kind::EqualEqual,
                    ComparisonOperator::BangEqual => Kind::BangEqual,
                });
                ComparisonOp {
                    operator,
                    token,
                    uuid: Uuid::new_v4(),
                }
            })
            .boxed()
    }
}
