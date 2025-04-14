use std::ops::Deref;

use crate::*;

impl Interpret for Variable {
    #[boxed]
    fn interpret(&self, _rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let vl = cx
            .values()
            .lookup(&self.ident)
            .await
            .map_err(|err| LinkedErr::from(err, self))?
            .ok_or(LinkedErr::from(
                E::UndefinedVariable(self.ident.clone()),
                self,
            ))?
            .deref()
            .clone();
        Ok(vl)
    }
}
