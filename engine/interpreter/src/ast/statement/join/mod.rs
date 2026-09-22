#[cfg(test)]
mod tests;

use crate::*;
use futures::future::join_all;
use tokio::spawn;

impl Interpret for Join {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let tasks = self
            .commands
            .iter()
            .cloned()
            .map(|node| {
                let branch_env = env.clone();
                spawn(async move { node.interpret(branch_env).await })
            })
            .collect::<Vec<_>>();
        // Await every branch and retain source order, including errors.
        let output = join_all(tasks)
            .await
            .into_iter()
            .map(|result| match result {
                Ok(Ok(value)) => value,
                Ok(Err(err)) => RtValue::Error(err.e.to_string()),
                Err(err) => RtValue::Error(E::from(err).to_string()),
            })
            .collect();
        Ok(RtValue::Vec(output))
    }
}
