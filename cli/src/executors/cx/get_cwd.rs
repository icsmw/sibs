use crate::{
    executors::{ExecutorPinnedResult, E},
    inf::{AnyValue, Context, Scope},
};

pub const NAME: &str = "get_cwd";

pub fn execute(_: Vec<AnyValue>, _cx: Context, sc: Scope) -> ExecutorPinnedResult {
    module_path!();
    Box::pin(async move {
        Ok(AnyValue::new(
            sc.get_cwd()
                .await?
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or(String::new()),
        ))
    })
}
