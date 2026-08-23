use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Determined(DeterminedTy::ExecuteResult))],
    DeterminedTy::Bool
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
    let FnEnv { args, caller, .. } = env;
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
