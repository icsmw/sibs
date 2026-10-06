use crate::*;

impl Interpret for EnvsImport {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        self.root.interpret(env).await
    }
}
