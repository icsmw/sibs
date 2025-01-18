#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Task {
    #[boxed]
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
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
}
