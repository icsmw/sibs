#[cfg(test)]
mod tests;

use crate::*;
use runtime::spawner;

impl Interpret for CommandPart {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Self::Literal(tk) => Ok(RtValue::Str(tk.to_string())),
            Self::Expression(_, n, _) => n.interpret(env).await,
            Self::Open(..) | Self::Close(..) => Ok(RtValue::Str(String::new())),
        }
    }
}

impl Interpret for Command {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let mut vls = Vec::new();
        for p in self.nodes.iter() {
            vls.push(
                p.interpret(env.clone())
                    .await?
                    .as_string()
                    .ok_or(LinkedErr::from(E::CannotBeConvertedToString, self))?,
            );
        }
        let cmd = vls.join("");
        spawner::spawn(
            &cmd,
            env.cx
                .cwd()
                .get()
                .await
                .map_err(|err| LinkedErr::from(err, self))?,
            env.job,
        )
        .await
        .map(|ss| ss.into())
        .map_err(|err| LinkedErr::from(err, self))
    }
}
