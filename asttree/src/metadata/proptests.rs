use crate::*;
use proptest::prelude::*;

impl Arbitrary for LinkedNode {
    type Parameters = Node;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(node: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            prop::strategy::Union::new(vec![Comment::arbitrary()
                .prop_map(|v| Node::Miscellaneous(Miscellaneous::Comment(v)))
                .boxed()])
            .prop_map(LinkedNode::from_node),
            0..2,
        )
        .prop_map(move |meta| {
            let mut md = Metadata {
                ppm: Vec::new(),
                meta,
            };
            let mut node = node.clone();
            match &mut node {
                Node::Expression(Expression::BinaryExp(ref mut n)) => {
                    md.take_meta(&mut n.left);
                }
                Node::Expression(Expression::BinaryExpSeq(ref mut n)) => {
                    if let Some(n) = n.nodes.first_mut() {
                        md.take_meta(n);
                    }
                }
                Node::Expression(Expression::Comparison(ref mut n)) => {
                    md.take_meta(&mut n.left);
                }
                Node::Expression(Expression::ComparisonSeq(ref mut n)) => {
                    if let Some(n) = n.nodes.first_mut() {
                        md.take_meta(n);
                    }
                }
                Node::Expression(Expression::Range(ref mut n)) => {
                    md.take_meta(&mut n.left);
                }
                Node::Expression(Expression::CompoundAssignments(ref mut n)) => {
                    md.take_meta(&mut n.left);
                }
                Node::Statement(Statement::Assignation(ref mut n)) => {
                    md.take_meta(&mut n.left);
                }
                Node::Statement(Statement::Optional(ref mut n)) => {
                    md.take_meta(&mut n.comparison);
                }
                Node::Declaration(Declaration::ArgumentDeclaration(ref mut n)) => {
                    md.take_meta(&mut n.variable);
                }
                _ => {}
            };
            LinkedNode { node, md }
        })
        .boxed()
    }
}
