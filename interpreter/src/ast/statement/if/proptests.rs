use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for IfCase {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(target: Self::Parameters) -> Self::Strategy {
        if target == 0 {
            (
                ComparisonSeq::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                    .boxed(),
                Block::arbitrary()
                    .prop_map(|v| Node::Statement(Statement::Block(v)))
                    .boxed(),
            )
                .prop_map(|(comp, blk)| IfCase::If(comp, blk, Token::for_test(Kind::If)))
                .boxed()
        } else {
            Block::arbitrary()
                .prop_map(|v| Node::Statement(Statement::Block(v)))
                .boxed()
                .prop_map(|blk| IfCase::Else(blk, Token::for_test(Kind::Else)))
                .boxed()
        }
    }
}

impl Arbitrary for If {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(IfCase::arbitrary_with(0), 1..5),
            prop::collection::vec(IfCase::arbitrary_with(0), 0..1),
        )
            .prop_map(move |(thrs, mut lst)| {
                let mut cases = Vec::new();
                for trh in thrs.into_iter() {
                    cases.push(trh);
                }

                if !lst.is_empty() {
                    cases.push(lst.remove(lst.len() - 1));
                }
                If { cases }
            })
            .boxed()
    }
}

test_node_reading!(r#if, If, 10);
