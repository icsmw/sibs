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

pub(crate) fn into_rt_ufns(mut fns: Fns) -> Fns {
    fns.ufns.funcs = fns
        .ufns
        .funcs
        .into_iter()
        .map(|(k, mut v)| {
            v.body = node_into_exec(v.body);
            (k, v)
        })
        .collect();
    fns.cfns.funcs = fns
        .cfns
        .funcs
        .into_iter()
        .map(|(k, mut v)| {
            v.body = node_into_exec(v.body);
            (k, v)
        })
        .collect();
    fns
}

fn node_into_exec(body: FnBody) -> FnBody {
    match body {
        FnBody::Executor(md, ex) => FnBody::Executor(md, ex),
        FnBody::Node(node) => {
            let meta = node.md.clone();
            let func = move |rt: Runtime| -> RtPinnedResult<LinkedErr<E>> {
                Box::pin({
                    let node = node.clone();
                    async move { node.interpret(rt).await }
                })
            };
            FnBody::Executor(meta, Box::new(func))
        }
        FnBody::Declaration => FnBody::Declaration,
    }
}

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

fn task_node_into_exec(body: TaskBody) -> TaskBody {
    match body {
        TaskBody::Executor(md, ex) => TaskBody::Executor(md, ex),
        TaskBody::Node(node) => {
            let meta = node.md.clone();
            let func = move |rt: Runtime| -> RtPinnedResult<LinkedErr<E>> {
                Box::pin({
                    let node = node.clone();
                    async move { node.interpret(rt).await }
                })
            };
            TaskBody::Executor(meta, Box::new(func))
        }
    }
}
