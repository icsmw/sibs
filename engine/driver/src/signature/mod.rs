#[cfg(test)]
mod tests;

use crate::*;

#[derive(Debug)]
pub struct Signature {
    pub name: String,
    pub signature: String,
    pub docs: Option<String>,
    pub args: Vec<String>,
    pub active: Option<usize>,
}

impl Signature {
    pub(crate) fn from_node(
        anchor: &Anchor,
        node: &LinkedNode,
        scx: Option<&SemanticCx>,
        pos: usize,
    ) -> Option<Self> {
        let is_func_call = |node: &LinkedNode| {
            matches!(
                node.get_node(),
                Node::Expression(Expression::FunctionCall(..))
            )
        };
        let node = if is_func_call(node) {
            node
        } else if let Some(node) = AnchorMap::map(anchor).find_parent(node.uuid(), is_func_call) {
            node
        } else {
            return None;
        };
        match node.get_node() {
            Node::Expression(Expression::FunctionCall(node)) => {
                let scx = scx?;
                let name = node.get_name();
                let func = scx.fns.find(&name)?;
                let active = node
                    .args
                    .iter()
                    .position(|arg| arg.get_position().is_in(pos))
                    .or_else(|| {
                        if node.args.is_empty() {
                            None
                        } else {
                            Some(node.args.len() - 1)
                        }
                    });
                let args = func
                    .args_desc()
                    .iter()
                    .map(|arg| format!("{}: {}", arg.name.as_ref().map_or("_", |v| v), arg.ty))
                    .collect::<Vec<String>>();
                let signature = format!("fn {name}({}) -> {}", args.join(", "), func.result_ty());
                Some(Signature {
                    name,
                    signature,
                    docs: func.docs(),
                    args,
                    active,
                })
            }
            _ => None,
        }
    }
}
