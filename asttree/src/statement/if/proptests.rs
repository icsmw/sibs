use crate::*;
use proptest::prelude::*;

impl Arbitrary for IfCase {
    type Parameters = (u8, u8);

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((target, deep): Self::Parameters) -> Self::Strategy {
        if target == 0 {
            (
                ComparisonSeq::arbitrary_with(deep + 1)
                    .prop_flat_map(|v| {
                        LinkedNode::arbitrary_with(Node::Expression(Expression::ComparisonSeq(v)))
                    })
                    .boxed(),
                Block::arbitrary_with(deep + 1)
                    .prop_flat_map(|v| {
                        LinkedNode::arbitrary_with(Node::Statement(Statement::Block(v)))
                    })
                    .boxed(),
            )
                .prop_map(|(comp, blk)| {
                    IfCase::If(comp, blk, Token::for_test(Kind::Keyword(Keyword::If)))
                })
                .boxed()
        } else {
            Block::arbitrary_with(deep + 1)
                .prop_flat_map(|v| LinkedNode::arbitrary_with(Node::Statement(Statement::Block(v))))
                .boxed()
                .prop_map(|blk| IfCase::Else(blk, Token::for_test(Kind::Keyword(Keyword::Else))))
                .boxed()
        }
    }
}

impl Arbitrary for If {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(IfCase::arbitrary_with((0, deep + 1)), 1..5),
            prop::collection::vec(IfCase::arbitrary_with((0, deep + 1)), 0..1),
        )
            .prop_map(move |(thrs, mut lst)| {
                let mut cases = Vec::new();
                for trh in thrs.into_iter() {
                    cases.push(trh);
                }

                if !lst.is_empty() {
                    cases.push(lst.remove(lst.len() - 1));
                }
                If {
                    cases,
                    uuid: Uuid::new_v4(),
                }
            })
            .boxed()
    }
}
