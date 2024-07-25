use crate::{
    elements::FuncArg,
    functions::ExecutorPinnedResult,
    inf::{operator, tools::get_last_name, AnyValue, Context, Scope},
};
use tokio::time::{sleep, Duration};

pub fn name() -> String {
    get_last_name(module_path!())
}

pub fn execute(
    args: Vec<FuncArg>,
    _args_token: usize,
    _cx: Context,
    _sc: Scope,
) -> ExecutorPinnedResult {
    Box::pin(async move {
        sleep(Duration::from_millis(
            args.first()
                .ok_or(operator::E::NoExpectedArgument)?
                .value
                .as_num()
                .ok_or(operator::E::FailToExtractValue)? as u64,
        ))
        .await;
        Ok(AnyValue::empty())
    })
}
