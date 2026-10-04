use crate::*;

impl Interpret for GlobalsModule {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        for node in &self.nodes {
            node.interpret(env.clone()).await?;
        }
        Ok(RtValue::Void)
    }
}
