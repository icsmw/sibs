use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Determined(DeterminedTy::Str))],
    DeterminedTy::Num
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(
    mut args: Vec<FnArgValue>,
    rt: Runtime,
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
    let Some(key) = arg.value.as_string() else {
        return Err(LinkedErr::by_link(
            E::InvalidFnArgumentType,
            (&caller).into(),
        ));
    };
    Ok(RtValue::Num(
        rt.signals()
            .waiters_signal(key)
            .await
            .map_err(|err| LinkedErr::by_link(err, (&caller).into()))? as f64,
    ))
}
