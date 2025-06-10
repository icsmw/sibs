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
    match status {
        ExecuteResult::Failed(code, ..) => {
            return Err(LinkedErr::by_link(
                E::SpawnFailed(format!(
                    "Command is failed with code: {}",
                    code.map(|c| c.to_string()).unwrap_or("unknown".to_owned())
                )),
                (&arg.link).into(),
            ))
        }
        ExecuteResult::RunError(err) => {
            return Err(LinkedErr::by_link(
                E::SpawnFailed(err.to_owned()),
                (&arg.link).into(),
            ))
        }
        _ => {}
    };
    Ok(arg.value)
}
