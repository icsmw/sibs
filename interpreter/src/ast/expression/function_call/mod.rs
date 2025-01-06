#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for FunctionCall {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let mut args = Vec::new();
        if let Some(parent) = rt
            .scopes
            .withdraw_parent_vl()
            .await
            .map_err(|err| LinkedErr::between(err, &self.open, &self.close))?
        {
            args.push(parent.into());
        }
        for n in self.args.iter() {
            args.push(FnArgValue::by_node(n.interpret(rt.clone()).await?, n));
        }
        let uuid = if let Some(RtValue::Closure(uuid)) = rt
            .scopes
            .lookup(self.get_name())
            .await
            .map_err(|err| LinkedErr::between(err, &self.open, &self.close))?
            .as_deref()
        {
            *uuid
        } else {
            self.uuid
        };
        rt.clone().fns.execute(&uuid, rt, args).await
    }
}
