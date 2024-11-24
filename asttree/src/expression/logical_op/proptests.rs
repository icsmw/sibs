use crate::*;
use proptest::prelude::*;

impl Arbitrary for LogicalOperator {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![Just(LogicalOperator::And), Just(LogicalOperator::Or),].boxed()
    }
}

impl Arbitrary for LogicalOp {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        LogicalOperator::arbitrary()
            .prop_map(|operator| {
                let token = Token::for_test(match &operator {
                    LogicalOperator::And => Kind::And,
                    LogicalOperator::Or => Kind::Or,
                });
                LogicalOp {
                    operator,
                    token,
                    uuid: Uuid::new_v4(),
                }
            })
            .boxed()
    }
}
