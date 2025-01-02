use std::ops::Deref;

use crate::*;

impl Interpret for Variable {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let vl = rt
            .scopes
            .lookup(&self.ident)
            .await
            .map_err(|err| LinkedErr::token(err, &self.token))?
            .ok_or(LinkedErr::token(
                E::UndefinedVariable(self.ident.clone()),
                &self.token,
            ))?
            .deref()
            .clone();
        Ok(vl)
    }
}
