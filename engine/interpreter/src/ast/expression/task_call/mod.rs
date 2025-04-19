#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for TaskCall {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let mut args = Vec::new();
        if let Some(parent) = cx
            .values()
            .withdraw_parent_vl()
            .await
            .map_err(|err| LinkedErr::from(err, self))?
        {
            args.push(parent.into());
        }
        for n in self.args.iter() {
            args.push(FnArgValue::by_node(
                n.interpret(rt.clone(), cx.clone()).await?,
                n,
            ));
        }
        let uuid = if let Some(RtValue::Closure(uuid)) = cx
            .values()
            .lookup(self.get_name())
            .await
            .map_err(|err| LinkedErr::from(err, self))?
            .as_deref()
        {
            *uuid
        } else {
            self.uuid
        };
        rt.clone()
            .tasks
            .execute(&uuid, rt, cx, args, &self.link())
            .await
    }
}
