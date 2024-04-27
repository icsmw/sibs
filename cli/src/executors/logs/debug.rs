use crate::{
    executors::ExecutorPinnedResult,
    inf::{AnyValue, Context, Scope},
};

pub const NAME: &str = "debug";

pub fn execute(msgs: Vec<AnyValue>, cx: Context, _sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        for msg in msgs.iter() {
            cx.journal.debug(
                String::new(),
                msg.get_as_string().unwrap_or(format!("{msg:?}")),
            );
        }
        Ok(AnyValue::new(()))
    })
}
