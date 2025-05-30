#[cfg(test)]
mod tests;

use crate::*;
use runtime::spawner;

impl Interpret for CommandPart {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Self::Literal(tk) => Ok(RtValue::Str(tk.to_string())),
            Self::Expression(_, n, _) => n.interpret(rt, cx).await,
            Self::Open(..) | Self::Close(..) => Ok(RtValue::Str(String::new())),
        }
    }
}

impl Interpret for Command {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let mut vls = Vec::new();
        for p in self.nodes.iter() {
            vls.push(
                p.interpret(rt.clone(), cx.clone())
                    .await?
                    .as_string()
                    .ok_or(LinkedErr::from(E::CannotBeConvertedToString, self))?,
            );
        }
        spawner::spawn(
            vls.join(""),
            cx.cwd()
                .get()
                .await
                .map_err(|err| LinkedErr::from(err, self))?,
            self.uuid,
            cx,
        )
        .await
        .map(|ss| ss.into())
        .map_err(|err| LinkedErr::from(err, self))
    }
}
