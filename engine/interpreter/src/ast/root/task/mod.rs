#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Task {
    #[boxed]
    fn interpret(&self, _env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}

impl Execute for Task {
    fn block(&self) -> &LinkedNode {
        &self.block
    }
    fn link(&self) -> SrcLink {
        self.slink()
    }
    fn uuid(&self) -> &Uuid {
        &self.uuid
    }
    #[boxed]
    fn before(&self, env: InterpreterEnvironment) -> GtPinnedResult<'_, LinkedErr<E>> {
        if self.gts.is_empty() {
            return Ok(true);
        }
        for gt in self.gts.iter() {
            let value = gt.interpret(env.clone()).await?;
            if let RtValue::Bool(value) = value {
                if !value {
                    return Ok(false);
                }
            } else {
                return Err(LinkedErr::from(
                    E::DismatchValueType(RtValueId::Bool.to_string(), value.id().to_string()),
                    gt,
                ));
            }
        }
        Ok(true)
    }
}
