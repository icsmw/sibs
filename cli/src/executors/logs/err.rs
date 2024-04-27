use crate::{executors::ExecutorPinnedResult, inf::any::AnyValue, inf::context::Context};

pub const NAME: &str = "err";

pub fn execute(msgs: Vec<AnyValue>, cx: Context) -> ExecutorPinnedResult {
    Box::pin(async move {
        for msg in msgs.iter() {
            cx.journal.err(
                "...".to_owned(),
                msg.get_as_string().unwrap_or(format!("{msg:?}")),
            );
        }
        Ok(AnyValue::new(()))
    })
}
