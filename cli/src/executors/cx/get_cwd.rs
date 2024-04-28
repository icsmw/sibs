use crate::{
    executors::{get_name, ExecutorPinnedResult},
    inf::{AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(_: Vec<AnyValue>, _cx: Context, sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        Ok(AnyValue::new(
            sc.get_cwd()
                .await?
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or(String::new()),
        ))
    })
}
