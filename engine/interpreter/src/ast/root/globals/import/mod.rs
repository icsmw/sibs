use crate::*;

impl Interpret for GlobalsImport {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        self.root.interpret(env).await
    }
}
