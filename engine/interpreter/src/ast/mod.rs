mod cfm;
mod declaration;
mod expression;
mod job;
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

        let InterpreterEnvironment { cx, .. } = env.clone();

        let mut vl = self.get_node().interpret(env.clone()).await?;
        let result = async {
            let mut linked_node = self;
            for ppm in self.get_md().ppm.iter() {
                cx.values()
                    .set_parent_vl(ParentValue::by_node(vl, linked_node))
                    .await
                    .map_err(link_err)?;
                vl = ppm.interpret(env.clone()).await?;
                linked_node = ppm;
            }
            Ok(vl)
        }
        .await;
        cx.values().drop_parent_vl().await.map_err(link_err)?;
        result
    }
}

impl Interpret for LinkedNode {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let link_err = |err: E| LinkedErr::by_link(err, (&self.get_md().link).into());

        let InterpreterEnvironment { job, .. } = env.clone();

        let owned_job = job
            .child(self.get_job_name(), self.get_visibility())
            .await
            .map_err(link_err)?;
        let owned_env = env.from_job(owned_job.clone());
        owned_job
            .start()
            .started(Some(self.get_job_name().to_string()))
            .await
            .map_err(link_err)?;

        let result = if owned_job.cancel().is_cancelled() {
            Err(link_err(E::Cancelled))
        } else {
            self.inner_interpret(owned_env).await
        };
        match result {
            Ok(vl) => {
                owned_job
                    .done()
                    .success(Some(vl.to_string()))
                    .await
                    .map_err(link_err)?;
                Ok(vl)
            }
            Err(err) if matches!(err.e, E::Cancelled) => {
                owned_job.cancel().cancelling().await.map_err(link_err)?;
                owned_job
                    .cancel()
                    .cancelled::<String>(None)
                    .await
                    .map_err(link_err)?;
                Err(err)
            }
            Err(err) => {
                owned_job
                    .done()
                    .failed(Some(err.e.to_string()))
                    .await
                    .map_err(link_err)?;
                Err(err)
            }
        }
    }
}
