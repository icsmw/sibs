use crate::{
    executors::{get_last_name, ExecutorPinnedResult},
    inf::{operator, AnyValue, Context, Scope},
};
use tokio::time::{sleep, Duration};

pub fn name() -> String {
    get_last_name(module_path!())
}

pub fn execute(args: Vec<AnyValue>, _cx: Context, _sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        sleep(Duration::from_millis(
            args.first()
                .ok_or(operator::E::NoExpectedArgument)?
                .as_num()
                .ok_or(operator::E::FailToExtractValue)? as u64,
        ))
        .await;
        Ok(AnyValue::empty())
    })
}
