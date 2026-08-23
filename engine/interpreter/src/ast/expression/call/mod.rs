#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Call {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        self.node.interpret(env).await
    }
}
