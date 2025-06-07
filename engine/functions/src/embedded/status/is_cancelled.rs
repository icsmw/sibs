use crate::*;

declare_embedded_fn!(
    vec![Ty::Determined(DeterminedTy::ExecuteResult)],
    DeterminedTy::Bool
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(
    args: Vec<FnArgValue>,
    _rt: Runtime,
    _cx: Context,
    caller: SrcLink,
) -> RtPinnedResult<'static, LinkedErr<E>> {
    let Some(arg) = args.first() else {
        return Err(LinkedErr::by_link(
            E::MissedFnArgument(RtValueId::ExecuteResult.to_string()),
            (&caller).into(),
        ));
    };
    let RtValue::ExecuteResult(status) = &arg.value else {
        return Err(LinkedErr::by_link(
            E::InvalidFnArgumentType,
            (&arg.link).into(),
        ));
    };
    Ok(RtValue::Bool(status.is_cancelled()))
}
