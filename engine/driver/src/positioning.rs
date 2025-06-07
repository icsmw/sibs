use crate::*;

pub enum Positioning {
    Function { name: String, docs: Option<String> },
}

impl Positioning {
    pub(crate) fn from_node(node: &LinkedNode, scx: Option<&SemanticCx>) -> Option<Self> {
        match node.get_node() {
            Node::Expression(Expression::FunctionCall(node)) => {
                let scx = scx?;
                let name = node.get_name();
                let func = scx.fns.find(&name)?;
                Some(Positioning::Function {
                    name,
                    docs: func.docs(),
                })
            }
            _ => None,
        }
    }
    pub fn docs(&self) -> Option<String> {
        match self {
            Self::Function { name, docs } => docs.clone(),
        }
    }
}
