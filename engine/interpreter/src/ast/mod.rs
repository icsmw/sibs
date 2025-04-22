mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;

impl Interpret for Node {
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.interpret(rt, cx),
            Node::Declaration(n) => n.interpret(rt, cx),
            Node::Expression(n) => n.interpret(rt, cx),
            Node::Miscellaneous(n) => n.interpret(rt, cx),
            Node::Root(n) => n.interpret(rt, cx),
            Node::Statement(n) => n.interpret(rt, cx),
            Node::Value(n) => n.interpret(rt, cx),
        }
    }
}

impl Interpret for LinkedNode {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let mut vl = self.get_node().interpret(rt.clone(), cx.clone()).await?;
        let mut linked_node = self;
        for ppm in self.get_md().ppm.iter() {
            cx.values()
                .set_parent_vl(ParentValue::by_node(vl, linked_node))
                .await
                .map_err(|err| LinkedErr::by_link(err, (&self.get_md().link).into()))?;
            vl = ppm.interpret(rt.clone(), cx.clone()).await?;
            linked_node = ppm;
        }
        cx.values()
            .drop_parent_vl()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.get_md().link).into()))?;
        Ok(vl)
    }
}
