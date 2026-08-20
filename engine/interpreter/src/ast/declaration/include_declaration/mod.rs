#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for IncludeDeclaration {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: ExecutionContext) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
