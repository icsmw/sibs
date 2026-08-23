use crate::*;

impl Interpret for ClosureDeclaration {
    #[boxed]
    fn interpret(&self, _env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Closure(self.uuid))
    }
}
