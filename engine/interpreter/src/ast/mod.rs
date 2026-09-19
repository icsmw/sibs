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

impl InterpretInner for LinkedNode {
    #[boxed]
    fn inner_interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let link_err = |err: E| LinkedErr::by_link(err, (&self.get_md().link).into());

        let InterpreterEnvironment { cx, job, .. } = env.clone();

        let node_env = env.from_job(job.child(self.get_node().id()).await.map_err(link_err)?);
        let mut vl = self.get_node().interpret(node_env.clone()).await?;
        let mut linked_node = self;
        for ppm in self.get_md().ppm.iter() {
            let ppm_env = env.from_job(job.child(self.get_node().id()).await.map_err(link_err)?);
            cx.values()
                .set_parent_vl(ParentValue::by_node(vl, linked_node))
                .await
                .map_err(link_err)?;
            vl = ppm.interpret(ppm_env.clone()).await?;
            linked_node = ppm;
        }
        cx.values().drop_parent_vl().await.map_err(link_err)?;
        Ok(vl)
    }
}
impl Interpret for LinkedNode {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let link_err = |err: E| LinkedErr::by_link(err, (&self.get_md().link).into());

        let InterpreterEnvironment { job, .. } = env.clone();

        job.start()
            .started(Some(self.get_node().id().to_string()))
            .await
            .map_err(link_err)?;

        match self.inner_interpret(env).await {
            Ok(vl) => {
                job.done()
                    .success(Some(vl.to_string()))
                    .await
                    .map_err(link_err)?;
                Ok(vl)
            }
            Err(err) => {
                job.done()
                    .failed(Some(err.e.to_string()))
                    .await
                    .map_err(link_err)?;
                Err(err)
            }
        }
    }
}
