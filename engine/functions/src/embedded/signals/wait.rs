use crate::*;

declare_embedded_fn!(
    vec![(None, None, Ty::Determined(DeterminedTy::Str))],
    DeterminedTy::Void
);

#[docs]
/// Documentation placeholder
#[boxed]
pub fn executor(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
    let FnEnv {
        mut args, caller, ..
    } = env;
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
    if let Some(tk) = env
        .rt
        .signals()
        .wait_signal(key)
        .await
        .map_err(|err| LinkedErr::by_link(err, (&caller).into()))?
    {
        if !tk.is_cancelled() {
            tokio::select! {
                _ = tk.cancelled() => {}
                _ = env.job.cancelled() => {}
            }
        }
    }
    Ok(RtValue::Void)
}
