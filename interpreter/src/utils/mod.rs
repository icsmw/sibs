use crate::*;

pub async fn chk_ty(node: &LinkedNode, vl: &RtValue, rt: &Runtime) -> Result<(), LinkedErr<E>> {
    let Some(ty) = rt
        .tys
        .get_ty(node.uuid())
        .await
        .map_err(|err| LinkedErr::by_node(err.into(), node))?
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
