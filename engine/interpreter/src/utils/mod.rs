use crate::*;

pub async fn chk_ty(node: &LinkedNode, vl: &RtValue, rt: &Runtime) -> Result<(), LinkedErr<E>> {
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

#[cfg(test)]
pub(crate) fn into_rt_ufns(mut fns: Fns) -> Fns {
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

#[cfg(test)]
fn ufn_into_exec(body: UserFnBody) -> UserFnBody {
    match body {
        UserFnBody::Executor(link, ex) => UserFnBody::Executor(link, ex),
        UserFnBody::Node(node) => {
            let link = node.slink();
            let func = move |rt: Runtime, cx: Context| -> RtPinnedResult<LinkedErr<E>> {
                Box::pin({
                    let node = node.clone();
                    async move { node.exec(rt, cx).await }
                })
            };
            UserFnBody::Executor(link, Box::new(func))
        }
        UserFnBody::Declaration => UserFnBody::Declaration,
    }
}

#[cfg(test)]
fn cfn_into_exec(body: ClosureFnBody) -> ClosureFnBody {
    match body {
        ClosureFnBody::Executor(link, ex) => ClosureFnBody::Executor(link, ex),
        ClosureFnBody::Node(node) => {
            let link = node.slink();
            let func = move |rt: Runtime, cx: Context| -> RtPinnedResult<LinkedErr<E>> {
                Box::pin({
                    let node = node.clone();
                    async move { node.exec(rt, cx).await }
                })
            };
            ClosureFnBody::Executor(link, Box::new(func))
        }
        ClosureFnBody::Declaration => ClosureFnBody::Declaration,
    }
}

#[cfg(test)]
pub(crate) fn into_rt_tasks(mut tasks: Tasks) -> Tasks {
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

#[cfg(test)]
fn task_node_into_exec(body: TaskBody) -> TaskBody {
    match body {
        TaskBody::Executor(md, ex) => TaskBody::Executor(md, ex),
        TaskBody::Node(node) => {
            let link = node.slink();
            let func = move |rt: Runtime, cx: Context| -> RtPinnedResult<LinkedErr<E>> {
                Box::pin({
                    let node = node.clone();
                    async move { node.exec(rt, cx).await }
                })
            };
            TaskBody::Executor(link, Box::new(func))
        }
    }
}

// #[cfg(test)]
// async fn runner(node: LinkedNode, rt: Runtime, cx: Context) -> Result<RtValue, LinkedErr<E>> {
//     cx.returns()
//         .open_cx(node.uuid())
//         .await
//         .map_err(|err| LinkedErr::from(err, &node))?;
//     let mut result = node.interpret(rt.clone(), cx.clone()).await?;
//     result = if let Some(result) = cx
//         .returns()
//         .withdraw_vl(node.uuid())
//         .await
//         .map_err(|err| LinkedErr::from(err, &node))?
//     {
//         result
//     } else {
//         result
//     };
//     cx.returns()
//         .close_cx()
//         .await
//         .map_err(|err| LinkedErr::from(err, &node))?;
//     Ok(result)
// }
