use crate::*;
use proptest::prelude::*;

impl Arbitrary for BinaryOperator {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(BinaryOperator::Slash),
            Just(BinaryOperator::Star),
            Just(BinaryOperator::Minus),
            Just(BinaryOperator::Plus),
        ]
        .boxed()
    }
}

impl Arbitrary for BinaryOp {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        BinaryOperator::arbitrary()
            .prop_map(|operator| {
                let token = Token::for_test(match &operator {
                    BinaryOperator::Slash => Kind::Slash,
                    BinaryOperator::Star => Kind::Star,
                    BinaryOperator::Minus => Kind::Minus,
                    BinaryOperator::Plus => Kind::Plus,
                });
                BinaryOp { operator, token }
            })
            .boxed()
    }
}
