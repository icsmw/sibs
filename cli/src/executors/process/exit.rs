use crate::{
    executors::{get_name, ExecutorPinnedResult},
    inf::{operator, AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(args: Vec<AnyValue>, cx: Context, _sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        cx.exit(
            if let Some(arg) = args.first() {
                arg.get_as_integer()
                    .ok_or(operator::E::FailToExtractValue)?
            } else {
                0
            } as i32,
            if let Some(arg) = args.get(1) {
                Some(arg.get_as_string().ok_or(operator::E::FailToExtractValue)?)
            } else {
                None
            },
        )
        .await?;
        Ok(AnyValue::new(()))
    })
}
