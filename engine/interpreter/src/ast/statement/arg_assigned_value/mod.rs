use crate::*;

impl Interpret for ArgumentAssignedValue {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let InterpreterEnvironment { rt, .. } = env.clone();
        let vl = self.node.interpret(env).await?;
        chk_ty(&self.node, &vl, &rt).await?;
        Ok(vl)
    }
}
