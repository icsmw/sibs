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
    if env.job.cancel().is_cancelled() {
        return Err(LinkedErr::by_link(E::Cancelled, (&caller).into()));
    }
    if let Some(tk) = env
        .rt
        .signals()
        .wait_signal(key)
        .await
        .map_err(|err| LinkedErr::by_link(err, (&caller).into()))?
    {
        if !tk.is_cancelled() {
            let cancel = env.job.cancel();
            tokio::select! {
                _ = tk.cancelled() => {}
                _ = cancel.cancellation() => {
                    return Err(LinkedErr::by_link(E::Cancelled, (&caller).into()));
                }
            }
        }
    }
    Ok(RtValue::Void)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn cancelled_wait_returns_cancellation_without_a_following_node() {
        use tokio::time::{timeout, Duration};
        let rt = Runtime::new(
            RtParameters::default_from_cwd().unwrap(),
            TypesTable::default(),
            Fns::default(),
            Tasks::default(),
        )
        .unwrap();
        let env = rt
            .create_interpreter_env("signal wait", None)
            .await
            .unwrap();
        let job = env.job.clone();
        job.start().started::<String>(None).await.unwrap();
        let call = FnEnv::from_interpreter_env(
            &env,
            vec![FnArgValue::new(
                RtValue::Str("Never".into()),
                SrcLink::default(),
            )],
            SrcLink::default(),
        );
        let execution = tokio::spawn(executor(call));
        timeout(Duration::from_secs(5), async {
            while rt.signals().waiters_signal("Never").await.unwrap() == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        job.cancel().cancelling().await.unwrap();
        let result = timeout(Duration::from_secs(5), execution)
            .await
            .unwrap()
            .unwrap();
        assert!(matches!(
            result,
            Err(LinkedErr {
                e: E::Cancelled,
                ..
            })
        ));
        job.cancel().cancelled::<String>(None).await.unwrap();
        rt.destroy().await.unwrap();
    }
}
