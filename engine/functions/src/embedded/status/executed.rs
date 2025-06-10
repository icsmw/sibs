use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Determined(DeterminedTy::ExecuteResult))],
    DeterminedTy::ExecuteResult
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(
    mut args: Vec<FnArgValue>,
    _rt: Runtime,
    _cx: Context,
    caller: SrcLink,
) -> RtPinnedResult<'static, LinkedErr<E>> {
    if args.len() != 1 {
        return Err(LinkedErr::by_link(
            E::MissedFnArgument(RtValueId::ExecuteResult.to_string()),
            (&caller).into(),
        ));
    }
    let arg = args.remove(0);
    let RtValue::ExecuteResult(status) = &arg.value else {
        return Err(LinkedErr::by_link(
            E::InvalidFnArgumentType,
            (&arg.link).into(),
        ));
    };
    if let ExecuteResult::RunError(err, ..) = status {
        return Err(LinkedErr::by_link(
            E::SpawnFailed(err.to_owned()),
            (&arg.link).into(),
        ));
    };
    Ok(arg.value)
}
