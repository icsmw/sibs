use crate::{
    executors::{get_name, ExecutorPinnedResult, E},
    inf::{AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(args: Vec<AnyValue>, cx: Context, sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        if args.len() != 1 {
            return Err(E::Executing(
                name(),
                "Expecting only one income argument as a name of signal".to_owned(),
            ));
        }
        let signal = args[0].as_string().ok_or(E::Executing(
            name(),
            "Cannot extract argument as string".to_owned(),
        ))?;
        let token = cx.signals.get(&signal).await?;
        cx.journal.debug(
            sc.get_current_task(),
            format!("waiting for signal \"{signal}\""),
        );
        token.cancelled().await;
        Ok(AnyValue::empty())
    })
}
