mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;

impl Interpret for Node {
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Node::ControlFlowModifier(n) => n.interpret(env),
            Node::Declaration(n) => n.interpret(env),
            Node::Expression(n) => n.interpret(env),
            Node::Miscellaneous(n) => n.interpret(env),
            Node::Root(n) => n.interpret(env),
            Node::Statement(n) => n.interpret(env),
            Node::Value(n) => n.interpret(env),
        }
    }
}

impl Interpret for LinkedNode {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let InterpreterEnvironment { cx, .. } = env.clone();
        let mut vl = self.get_node().interpret(env.clone()).await?;
        let mut linked_node = self;
        for ppm in self.get_md().ppm.iter() {
            cx.values()
                .set_parent_vl(ParentValue::by_node(vl, linked_node))
                .await
                .map_err(|err| LinkedErr::by_link(err, (&self.get_md().link).into()))?;
            vl = ppm.interpret(env.clone()).await?;
            linked_node = ppm;
        }
        cx.values()
            .drop_parent_vl()
            .await
            .map_err(|err| LinkedErr::by_link(err, (&self.get_md().link).into()))?;
        Ok(vl)
    }
}
