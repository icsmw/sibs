use crate::{
    elements::FuncArg,
    error::LinkedErr,
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, AnyValue, Context, Scope},
};
use std::path::PathBuf;

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(
    paths: Vec<FuncArg>,
    args_token: usize,
    _cx: Context,
    sc: Scope,
) -> ExecutorPinnedResult {
    module_path!();
    Box::pin(async move {
        if paths.len() != 1 {
            Err(LinkedErr::new(
                E::Executing(
                    name(),
                    "Expecting only one income argument as a CWD".to_owned(),
                ),
                Some(args_token),
            ))?;
        }
        let path = PathBuf::from(paths[0].value.as_string().ok_or(paths[0].err(E::Executing(
            name(),
            "Cannot extract argument as string".to_owned(),
        )))?);
        let path = if path.is_absolute() {
            path
        } else if let Some(cwd) = sc.get_cwd().await? {
            cwd.join(path)
        } else {
            return Err(paths[0].err(E::Executing(
                name(),
                format!(
                    "Cannot switch to relative \"{}\" because CWD isn't setup",
                    path.to_string_lossy()
                ),
            )));
        };
        if !path.exists() {
            return Err(paths[0].err(E::Executing(
                name(),
                format!("Folder {} doesn't exist", path.to_string_lossy()),
            )));
        }
        sc.set_cwd(Some(path.clone())).await?;
        Ok(AnyValue::new(path)?)
    })
}
