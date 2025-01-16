use crate::*;

#[derive(Debug, Clone)]
pub enum NodeTarget<'a> {
    Statement(&'a [StatementId]),
    Expression(&'a [ExpressionId]),
    Declaration(&'a [DeclarationId]),
    Value(&'a [ValueId]),
    ControlFlowModifier(&'a [ControlFlowModifierId]),
    Root(&'a [RootId]),
    Miscellaneous(&'a [MiscellaneousId]),
}

impl<'a> NodeTarget<'a> {
    pub fn is(&self, node: &Node) -> bool {
        let id = node.id();
        match self {
            Self::Statement(ids) if id == NodeId::Statement => {
                if let Node::Statement(node) = node {
                    ids.contains(&node.id())
                } else {
                    false
                }
            }
            Self::Expression(ids) if id == NodeId::Expression => {
                if let Node::Expression(node) = node {
                    ids.contains(&node.id())
                } else {
                    false
                }
            }
            Self::Declaration(ids) if id == NodeId::Declaration => {
                if let Node::Declaration(node) = node {
                    ids.contains(&node.id())
                } else {
                    false
                }
            }
            Self::Value(ids) if id == NodeId::Value => {
                if let Node::Value(node) = node {
                    ids.contains(&node.id())
                } else {
                    false
                }
            }
            Self::ControlFlowModifier(ids) if id == NodeId::ControlFlowModifier => {
                if let Node::ControlFlowModifier(node) = node {
                    ids.contains(&node.id())
                } else {
                    false
                }
            }
            Self::Root(ids) if id == NodeId::Root => {
                if let Node::Root(node) = node {
                    ids.contains(&node.id())
                } else {
                    false
                }
            }
            Self::Miscellaneous(ids) if id == NodeId::Miscellaneous => {
                if let Node::Miscellaneous(node) = node {
                    ids.contains(&node.id())
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

pub trait FilterTargetsFromNodes<'a> {
    fn into_filtered_nodes(self, trgs: &[NodeTarget]) -> Vec<&'a LinkedNode>;
}

impl<'a> FilterTargetsFromNodes<'a> for Vec<&'a LinkedNode> {
    fn into_filtered_nodes(self, trgs: &[NodeTarget]) -> Vec<&'a LinkedNode> {
        self.into_iter()
            .filter(|n| trgs.iter().any(|trg| trg.is(&n.node)))
            .map(|n| n)
            .collect::<Vec<&LinkedNode>>()
    }
}

impl<'a> FilterTargetsFromNodes<'a> for &'a LinkedNode {
    fn into_filtered_nodes(self, trgs: &[NodeTarget]) -> Vec<&'a LinkedNode> {
        if trgs.iter().any(|trg| trg.is(&self.node)) {
            vec![self]
        } else {
            Vec::new()
        }
    }
}

impl<'a> From<NodeTarget<'a>> for NodeId {
    fn from(trg: NodeTarget) -> Self {
        match trg {
            NodeTarget::ControlFlowModifier(..) => NodeId::ControlFlowModifier,
            NodeTarget::Statement(..) => NodeId::Statement,
            NodeTarget::Expression(..) => NodeId::Expression,
            NodeTarget::Declaration(..) => NodeId::Declaration,
            NodeTarget::Value(..) => NodeId::Value,
            NodeTarget::Root(..) => NodeId::Root,
            NodeTarget::Miscellaneous(..) => NodeId::Miscellaneous,
        }
    }
}

impl<'a> From<&NodeTarget<'a>> for NodeId {
    fn from(trg: &NodeTarget) -> Self {
        match trg {
            NodeTarget::ControlFlowModifier(..) => NodeId::ControlFlowModifier,
            NodeTarget::Statement(..) => NodeId::Statement,
            NodeTarget::Expression(..) => NodeId::Expression,
            NodeTarget::Declaration(..) => NodeId::Declaration,
            NodeTarget::Value(..) => NodeId::Value,
            NodeTarget::Root(..) => NodeId::Root,
            NodeTarget::Miscellaneous(..) => NodeId::Miscellaneous,
        }
    }
}
