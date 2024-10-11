use crate::{
    elements::{Element, Join},
    error::LinkedErr,
    inf::{
        operator::E, Execute, ExecuteContext, ExecutePinnedResult, ExecuteResult, Processing,
        TryExecute, Value,
    },
};
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::{
    spawn,
    task::{JoinError, JoinHandle},
};
use tokio_util::sync::CancellationToken;

enum TaskError {
    Join(JoinError),
}

impl From<TaskError> for LinkedErr<E> {
    fn from(err: TaskError) -> Self {
        match err {
            TaskError::Join(err) => E::JoinError(err.to_string()).unlinked(),
        }
    }
}

impl Processing for Join {}

impl TryExecute for Join {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        async fn wait(
            tasks: &mut [JoinHandle<ExecuteResult>],
            token: CancellationToken,
        ) -> Result<Vec<Result<Value, LinkedErr<E>>>, TaskError> {
            let mut results: Vec<Result<Value, LinkedErr<E>>> = Vec::new();
            let mut futures = FuturesUnordered::new();
            for task in tasks {
                futures.push(task);
            }
            while let Some(result) = futures.next().await {
                match result {
                    Ok(Ok(result)) => {
                        results.push(Ok(result));
                    }
                    Ok(Err(err)) => {
                        if !token.is_cancelled() {
                            token.cancel();
                        }
                        results.push(Err(err));
                    }
                    Err(err) => {
                        return Err(TaskError::Join(err));
                    }
                }
            }
            Ok(results)
        }
        Box::pin(async move {
            let Element::Values(values, _) = self.elements.as_ref() else {
                return Ok(Value::empty());
            };
            let mut tasks = values
                .elements
                .iter()
                .cloned()
                .map(|el| {
                    let props = cx.split();
                    spawn(async move {
                        let inner = ExecuteContext::join(&props.0, props.1);
                        // inside exclude will be create clone
                        el.execute(inner).await
                    })
                })
                .collect::<Vec<JoinHandle<ExecuteResult>>>();
            match wait(&mut tasks, cx.token).await {
                Ok(results) => {
                    let mut output: Vec<Value> = Vec::new();
                    for result in results.into_iter() {
                        match result {
                            Ok(value) => {
                                output.push(value);
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        };
                    }
                    Ok(Value::Vec(output))
                }
                Err(err) => Err(err.into()),
            }
        })
    }
}
