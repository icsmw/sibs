use crate::*;

declare_embedded_fn!(vec![Ty::Determined(DeterminedTy::Str)], DeterminedTy::Void);

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
    if let Some(tk) = rt
        .signals()
        .wait_signal(key)
        .await
        .map_err(|err| LinkedErr::by_link(err, (&caller).into()))?
    {
        if !tk.is_cancelled() {
            tk.cancelled().await;
        }
    }
    Ok(RtValue::Void)
}
