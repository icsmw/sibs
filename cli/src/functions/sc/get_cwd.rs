use crate::{
    elements::FuncArg,
    functions::ExecutorPinnedResult,
    inf::{tools::get_name, AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(
    _: Vec<FuncArg>,
    _args_token: usize,
    _cx: Context,
    sc: Scope,
) -> ExecutorPinnedResult {
    Box::pin(async move { Ok(AnyValue::PathBuf(sc.get_cwd().await?)) })
}
