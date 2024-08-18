use crate::{
    elements::FuncArg,
    error::LinkedErr,
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, Value, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(
    args: Vec<FuncArg>,
    args_token: usize,
    cx: Context,
    sc: Scope,
) -> ExecutorPinnedResult {
    Box::pin(async move {
        if args.len() != 1 {
            return Err(LinkedErr::new(
                E::Executing(
                    name(),
                    "Expecting only one income argument as a name of signal".to_owned(),
                ),
                Some(args_token),
            ))?;
        }
        let signal = args[0].value.as_string().ok_or(args[0].err(E::Executing(
            name(),
            "Cannot extract argument as string".to_owned(),
        )))?;
        let token = cx.signals.get(&signal).await?;
        sc.journal.debug(format!("waiting for signal \"{signal}\""));
        token.cancelled().await;
        Ok(Value::empty())
    })
}
