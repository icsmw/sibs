use std::ops::Deref;

use crate::*;

impl Interpret for Variable {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let InterpreterEnvironment { cx, .. } = env;
        let local = cx
            .values()
            .lookup(&self.ident)
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        let vl = match local {
            Some(value) => Some(value),
            None => cx
                .globals()
                .lookup(&self.ident)
                .await
                .map_err(|err| LinkedErr::from(err, self))?,
        }
        .ok_or(LinkedErr::from(
            E::UndefinedVariable(self.ident.clone()),
            self,
        ))?
        .deref()
        .clone();
        if self.negation.is_some() {
            return match vl {
                RtValue::Bool(value) => Ok(RtValue::Bool(!value)),
                _ => Err(LinkedErr::from(E::InvalidValueType("bool".into()), self)),
            };
        }
        Ok(vl)
    }
}
