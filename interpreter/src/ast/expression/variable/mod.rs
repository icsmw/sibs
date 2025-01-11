use std::ops::Deref;

use crate::*;

impl Interpret for Variable {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let vl = rt
            .scopes
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
