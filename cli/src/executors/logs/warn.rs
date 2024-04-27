use crate::{
    executors::ExecutorPinnedResult,
    inf::{AnyValue, Context, Scope},
};

pub const NAME: &str = "warn";

pub fn execute(msgs: Vec<AnyValue>, cx: Context, sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        for msg in msgs.iter() {
            cx.journal.warn(
                "...".to_owned(),
                msg.get_as_string().unwrap_or(format!("{msg:?}")),
            );
        }
        Ok(AnyValue::new(()))
    })
}
