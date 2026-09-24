use crate::*;
use futures::FutureExt;
use std::{future::Future, panic::AssertUnwindSafe};

pub(crate) async fn catch_execution_panic<F>(
    future: F,
    link: SrcLink,
) -> Result<RtValue, LinkedErr<E>>
where
    F: Future<Output = Result<RtValue, LinkedErr<E>>>,
{
    // The panicking future is discarded, never polled again. Returning an error
    // lets callers run their normal async cleanup before completing their jobs.
    match AssertUnwindSafe(future).catch_unwind().await {
        Ok(result) => result,
        Err(payload) => {
            let message = if let Some(message) = payload.downcast_ref::<String>() {
                message.clone()
            } else if let Some(message) = payload.downcast_ref::<&str>() {
                (*message).to_owned()
            } else {
                "non-string panic payload".to_owned()
            };
            Err(LinkedErr::by_link(
                E::ExecutionPanicked(message),
                (&link).into(),
            ))
        }
    }
}

pub(crate) async fn chk_ty(
    node: &LinkedNode,
    vl: &RtValue,
    rt: &Runtime,
) -> Result<(), LinkedErr<E>> {
    let Some(ty) = rt.tys.get(node.uuid()) else {
        return Err(LinkedErr::from(E::FailInferType, node));
    };
    if !ty.reassignable(
        &vl.as_ty()
            .ok_or(LinkedErr::from(E::NotPublicValueType, node))?,
    ) {
        Err(LinkedErr::from(E::InvalidValueType(ty.to_string()), node))
    } else {
        Ok(())
    }
}

fn into_rt_ufns(mut fns: Fns) -> Fns {
    fns.ufns.funcs = fns
        .ufns
        .funcs
        .into_iter()
        .map(|(k, mut v)| {
            v.body = ufn_into_exec(v.body);
            (k, v)
        })
        .collect();
    fns.cfns.funcs = fns
        .cfns
        .funcs
        .into_iter()
        .map(|(k, mut v)| {
            v.body = cfn_into_exec(v.body);
            (k, v)
        })
        .collect();
    fns
}

async fn execution<N>(
    env: InterpreterEnvironment,
    node: &N,
    name: Option<String>,
) -> Result<RtValue, LinkedErr<E>>
where
    N: Execute + SrcLinking,
{
    env.job
        .start()
        .started(name)
        .await
        .map_err(|err| LinkedErr::from(err, node))?;

    let result = if env.job.cancel().is_cancelled() {
        Err(LinkedErr::from(E::Cancelled, node))
    } else {
        catch_execution_panic(async { node.exec(env.clone()).await }, node.slink()).await
    };

    match &result {
        Ok(_) => {
            env.job
                .done()
                .success::<String>(None)
                .await
                .map_err(|err| LinkedErr::from(err, node))?;
        }
        Err(err) if matches!(err.e, E::Cancelled) => {
            env.job
                .cancel()
                .cancelling()
                .await
                .map_err(|err| LinkedErr::from(err, node))?;
            env.job
                .cancel()
                .cancelled::<String>(None)
                .await
                .map_err(|err| LinkedErr::from(err, node))?;
        }
        Err(err) => {
            env.job
                .done()
                .failed::<String>(Some(err.e.to_string()))
                .await
                .map_err(|err| LinkedErr::from(err, node))?;
        }
    }
    result
}
fn ufn_into_exec(body: UserFnBody) -> UserFnBody {
    match body {
        UserFnBody::Executor(link, ex) => UserFnBody::Executor(link, ex),
        UserFnBody::Node(node) => {
            let link = node.slink();
            let func = move |env: InterpreterEnvironment| -> RtPinnedResult<LinkedErr<E>> {
                Box::pin({
                    let node = node.clone();
                    async move { execution(env, &node, None).await }
                })
            };
            UserFnBody::Executor(link, Box::new(func))
        }
        UserFnBody::Declaration => UserFnBody::Declaration,
    }
}

fn cfn_into_exec(body: ClosureFnBody) -> ClosureFnBody {
    match body {
        ClosureFnBody::Executor(link, ex) => ClosureFnBody::Executor(link, ex),
        ClosureFnBody::Node(node) => {
            let link = node.slink();
            let func = move |env: InterpreterEnvironment| -> RtPinnedResult<LinkedErr<E>> {
                Box::pin({
                    let node = node.clone();
                    async move { execution(env, &node, None).await }
                })
            };
            ClosureFnBody::Executor(link, Box::new(func))
        }
        ClosureFnBody::Declaration => ClosureFnBody::Declaration,
    }
}

fn into_rt_tasks(mut tasks: Tasks) -> Tasks {
    tasks.table = tasks
        .table
        .into_iter()
        .map(|(k, mut v)| {
            v.body = task_node_into_exec(v.body);
            (k, v)
        })
        .collect();
    tasks
}

fn task_node_into_exec(body: TaskBody) -> TaskBody {
    match body {
        TaskBody::Executor(md, ex) => TaskBody::Executor(md, ex),
        TaskBody::Node(node) => {
            let link = node.slink();
            let func = move |env: InterpreterEnvironment| -> RtPinnedResult<LinkedErr<E>> {
                Box::pin({
                    let node = node.clone();
                    async move { execution(env, &node, Some(node.get_name())).await }
                })
            };
            TaskBody::Executor(link, Box::new(func))
        }
    }
}

pub fn runtime(params: RtParameters, scx: SemanticCx) -> Result<Runtime, E> {
    Runtime::new(
        params,
        scx.table,
        into_rt_ufns(scx.fns),
        into_rt_tasks(scx.tasks),
    )
}
