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
        node: &LinkedNode,
        scx: Option<&SemanticCx>,
        pos: usize,
    ) -> Option<Self> {
        match node.get_node() {
            Node::Expression(Expression::FunctionCall(node)) => {
                let scx = scx?;
                let name = node.get_name();
                let func = scx.fns.find(&name)?;
                let active = node
                    .args
                    .iter()
                    .position(|arg| arg.get_position().is_in(pos))
                    .unwrap_or(node.args.len());
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
                    active: Some(active),
                })
            }
            _ => None,
        }
    }
}
