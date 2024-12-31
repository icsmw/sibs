use crate::*;

pub async fn chk_ty(node: &LinkedNode, vl: &RtValue, rt: &Runtime) -> Result<(), LinkedErr<E>> {
    let Some(ty) = rt
        .tys
        .get_ty(node.uuid())
        .await
        .map_err(|err| LinkedErr::by_node(err, node))?
    else {
        return Err(LinkedErr::by_node(E::FailInferType, node));
    };
    if !ty.reassignable(
        &vl.as_ty()
            .ok_or(LinkedErr::by_node(E::NotPublicValueType, node))?,
    ) {
        Err(LinkedErr::by_node(
            E::InvalidValueType(ty.to_string()),
            node,
        ))
    } else {
        Ok(())
    }
}

pub(crate) fn into_rt_fns(mut fns: Fns) -> Fns {
    fns.funcs = fns
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
    }
}
