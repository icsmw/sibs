#[cfg(test)]
mod tests;

use crate::*;
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;
use tokio::{spawn, task::JoinHandle};

type LinkedJoinHandle = (SrcLink, JoinHandle<(Uuid, Result<RtValue, LinkedErr<E>>)>);

async fn wait(
    tasks: Vec<LinkedJoinHandle>,
    job: &Job,
) -> Result<HashMap<Uuid, Result<RtValue, LinkedErr<E>>>, LinkedErr<E>> {
    let mut results: HashMap<Uuid, Result<RtValue, LinkedErr<E>>> = HashMap::new();
    let mut futures = FuturesUnordered::new();
    for (link, task) in tasks {
        futures.push(async move { task.await.map_err(|err| (link, err)) });
    }
    while let Some(result) = futures.next().await {
        match result {
            Ok((uuid, Ok(result))) => {
                results.insert(uuid, Ok(result));
            }
            Ok((uuid, Err(err))) => {
                if !job.is_cancelled() {
                    job.cancel().failed(Some(err.e.to_string()));
                }
                results.insert(uuid, Err(err));
            }
            Err((link, err)) => {
                return Err(LinkedErr::by_link(err.into(), (&link).into()));
            }
        }
    }
    Ok(results)
}

impl Interpret for Join {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let InterpreterEnvironment { rt, cx, job } = env.clone();
        let join_env = InterpreterEnvironment {
            rt: rt.clone(),
            cx: cx.clone(),
            job: job
                .child(Uuid::new_v4(), "join")
                .await
                .map_err(|err| LinkedErr::by_link(err, (&self.link()).into()))?,
        };
        let order = self
            .commands
            .iter()
            .map(|node| *node.uuid())
            .collect::<Vec<Uuid>>();
        let tasks = self
            .commands
            .iter()
            .cloned()
            .map(|node| {
                let join_env_inner = join_env.clone();
                (
                    node.link(),
                    spawn(async move { (*node.uuid(), node.interpret(join_env_inner).await) }),
                )
            })
            .collect::<Vec<LinkedJoinHandle>>();
        let result = wait(tasks, &join_env.job).await;
        join_env.job.close();
        match result {
            Ok(mut results) => {
                if order.len() != results.len() {
                    return Err(LinkedErr::by_link(
                        E::SomeNodesHadSameUuid,
                        (&self.link()).into(),
                    ));
                }
                let mut output: Vec<RtValue> = Vec::new();
                for uuid in order.into_iter() {
                    match results.remove(&uuid) {
                        Some(Ok(value)) => {
                            output.push(value);
                        }
                        Some(Err(err)) => {
                            return Err(err);
                        }
                        None => {
                            return Err(LinkedErr::by_link(
                                E::FailToFindJoinResult(uuid),
                                (&self.link()).into(),
                            ));
                        }
                    }
                }
                Ok(RtValue::Vec(output))
            }
            Err(err) => Err(err),
        }
    }
}
