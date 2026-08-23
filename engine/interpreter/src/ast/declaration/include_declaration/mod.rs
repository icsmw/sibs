#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for IncludeDeclaration {
    #[boxed]
    fn interpret(&self, _env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
