mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;

impl Interpret for Node {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.interpret(rt),
            Node::Declaration(n) => n.interpret(rt),
            Node::Expression(n) => n.interpret(rt),
            Node::Miscellaneous(n) => n.interpret(rt),
            Node::Root(n) => n.interpret(rt),
            Node::Statement(n) => n.interpret(rt),
            Node::Value(n) => n.interpret(rt),
        }
    }
}

impl Interpret for LinkedNode {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let mut vl = self.node.interpret(rt.clone()).await?;
        let mut linked_node = self;
        for ppm in self.md.ppm.iter() {
            rt.scopes
                .set_parent_vl(ParentValue::by_node(vl, linked_node))
                .await
                .map_err(|err| LinkedErr::by_link(err, (&self.md.link).into()))?;
            vl = ppm.interpret(rt.clone()).await?;
            linked_node = ppm;
        }
        rt.scopes
            .drop_parent_vl()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.md.link).into()))?;
        Ok(vl)
    }
}
