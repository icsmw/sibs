use crate::*;
use proptest::prelude::*;

// impl Arbitrary for LinkedNode {
//     type Parameters = Node;

//     type Strategy = BoxedStrategy<Self>;

//     fn arbitrary_with(node: Self::Parameters) -> Self::Strategy {
//         prop::collection::vec(
//             prop::strategy::Union::new(vec![
//                 Comment::arbitrary()
//                     .prop_map(|v| Node::Miscellaneous(Miscellaneous::Comment(v)))
//                     .boxed(),
//                 Meta::arbitrary()
//                     .prop_map(|v| Node::Miscellaneous(Miscellaneous::Meta(v)))
//                     .boxed(),
//             ])
//             .prop_map(LinkedNode::from_node),
//             0..4,
//         )
//         .prop_map(move |meta| {
//             let mut md = Metadata {
//                 ppm: Vec::new(),
//                 meta,
//             };
//             let mut node = node.clone();
//             match &mut node {
//                 // Processing meta
//                 Node::Expression(Expression::BinaryExp(ref mut n)) => {
//                     md.take_meta(&mut n.left);
//                 }
//                 Node::Expression(Expression::BinaryExpSeq(ref mut n)) => {
//                     if let Some(n) = n.nodes.first_mut() {
//                         md.take_meta(n);
//                     }
//                 }
//                 Node::Expression(Expression::Comparison(ref mut n)) => {
//                     md.take_meta(&mut n.left);
//                 }
//                 Node::Expression(Expression::ComparisonSeq(ref mut n)) => {
//                     if let Some(n) = n.nodes.first_mut() {
//                         md.take_meta(n);
//                     }
//                 }
//                 Node::Expression(Expression::Range(ref mut n)) => {
//                     md.take_meta(&mut n.left);
//                 }
//                 Node::Expression(Expression::CompoundAssignments(ref mut n)) => {
//                     md.take_meta(&mut n.left);
//                 }
//                 Node::Statement(Statement::Assignation(ref mut n)) => {
//                     md.take_meta(&mut n.left);
//                 }
//                 Node::Statement(Statement::Optional(ref mut n)) => {
//                     md.take_meta(&mut n.comparison);
//                 }
//                 Node::Declaration(Declaration::ArgumentDeclaration(ref mut n)) => {
//                     md.take_meta(&mut n.variable);
//                 }
//                 _ => {}
//             };
//             LinkedNode { node, md }
//         })
//         .boxed()
//     }
// }

fn resolve_meta(node: &mut Node, md: &mut Metadata) {
    match node {
        // Processing meta
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
}

impl Arbitrary for LinkedNode {
    type Parameters = (Node, u8);

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((node, deep): Self::Parameters) -> Self::Strategy {
        let meta_strategy = prop::collection::vec(
            prop::strategy::Union::new(vec![
                Comment::arbitrary()
                    .prop_map(|v| Node::Miscellaneous(Miscellaneous::Comment(v)))
                    .boxed(),
                Meta::arbitrary()
                    .prop_map(|v| Node::Miscellaneous(Miscellaneous::Meta(v)))
                    .boxed(),
            ])
            .prop_map(LinkedNode::from_node),
            0,
        );
        if !matches!(
            node,
            Node::Expression(Expression::Variable(..))
                | Node::Statement(Statement::ArgumentAssignation(..))
                | Node::Statement(Statement::ArgumentAssignedValue(..))
                | Node::Expression(Expression::FunctionCall(..))
                | Node::Expression(Expression::Command(..))
                | Node::Value(Value::Array(..))
                | Node::Value(Value::InterpolatedString(..))
        ) || deep > PROPTEST_DEEP_FACTOR
        {
            // PPM isn't required
            meta_strategy
                .prop_map(move |meta| {
                    let mut md = Metadata {
                        ppm: Vec::new(),
                        meta,
                        link: SrcLink::default(),
                    };
                    let mut node = node.clone();
                    resolve_meta(&mut node, &mut md);
                    LinkedNode { node, md }
                })
                .boxed()
        } else {
            // PPM is required
            (
                meta_strategy,
                prop::collection::vec(
                    prop::strategy::Union::new(vec![
                        Accessor::arbitrary_with(deep + PROPTEST_DEEP_FACTOR)
                            .prop_map(|v| Node::Expression(Expression::Accessor(v)))
                            .boxed(),
                        Call::arbitrary_with(deep + PROPTEST_DEEP_FACTOR)
                            .prop_map(|v| Node::Expression(Expression::Call(v)))
                            .boxed(),
                    ])
                    .prop_map(LinkedNode::from_node),
                    0..2,
                ),
            )
                .prop_map(move |(meta, ppm)| {
                    let mut md = Metadata {
                        ppm,
                        meta,
                        link: SrcLink::default(),
                    };
                    let mut node = node.clone();
                    resolve_meta(&mut node, &mut md);
                    LinkedNode { node, md }
                })
                .boxed()
        }
        .boxed()
    }
}
