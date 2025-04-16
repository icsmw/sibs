#[cfg(test)]
mod tests;

use crate::*;
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;
use tokio::{spawn, task::JoinHandle};
use tokio_util::sync::CancellationToken;

type LinkedJoinHandle = (SrcLink, JoinHandle<(Uuid, Result<RtValue, LinkedErr<E>>)>);

async fn wait(
    tasks: Vec<LinkedJoinHandle>,
    token: CancellationToken,
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
                if !token.is_cancelled() {
                    token.cancel();
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
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
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
                let (rt, cx) = (rt.clone(), cx.clone());
                (
                    node.link(),
                    spawn(async move { (*node.uuid(), node.interpret(rt, cx).await) }),
                )
            })
            .collect::<Vec<LinkedJoinHandle>>();
        match wait(tasks, cx.job.cancel).await {
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
