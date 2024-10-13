use crate::elements::{Element, ElementId, IfThread};
use proptest::prelude::*;

impl Arbitrary for IfThread {
    type Parameters = (u8, usize);
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((target, deep): Self::Parameters) -> Self::Strategy {
        if target == 0 {
            (
                Element::arbitrary_with((
                    vec![ElementId::IfSubsequence, ElementId::IfCondition],
                    deep,
                )),
                Element::arbitrary_with((vec![ElementId::Block], deep)),
            )
                .prop_map(|(subsequence, block)| {
                    IfThread::If(Box::new(subsequence), Box::new(block))
                })
                .boxed()
        } else {
            Element::arbitrary_with((vec![ElementId::Block], deep))
                .prop_map(|block| IfThread::Else(Box::new(block)))
                .boxed()
        }
    }
}
