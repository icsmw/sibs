use crate::*;
use proptest::prelude::*;

impl Arbitrary for CompoundAssignmentsOperator {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(CompoundAssignmentsOperator::PlusEqual),
            Just(CompoundAssignmentsOperator::MinusEqual),
            Just(CompoundAssignmentsOperator::SlashEqual),
            Just(CompoundAssignmentsOperator::StarEqual),
        ]
        .boxed()
    }
}

impl Arbitrary for CompoundAssignmentsOp {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        CompoundAssignmentsOperator::arbitrary()
            .prop_map(|operator| {
                let token = Token::for_test(match &operator {
                    CompoundAssignmentsOperator::PlusEqual => Kind::PlusEqual,
                    CompoundAssignmentsOperator::MinusEqual => Kind::MinusEqual,
                    CompoundAssignmentsOperator::SlashEqual => Kind::SlashEqual,
                    CompoundAssignmentsOperator::StarEqual => Kind::StarEqual,
                });
                CompoundAssignmentsOp {
                    operator,
                    token,
                    uuid: Uuid::new_v4(),
                }
            })
            .boxed()
    }
}
