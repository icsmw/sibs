use crate::{
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, AnyValue, Context, Scope},
};
use std::path::PathBuf;

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(path: Vec<AnyValue>, _cx: Context, sc: Scope) -> ExecutorPinnedResult {
    module_path!();
    Box::pin(async move {
        if path.len() != 1 {
            return Err(E::Executing(
                name(),
                "Expecting only one income argument as a CWD".to_owned(),
            ));
        }
        let path = PathBuf::from(path[0].as_string().ok_or(E::Executing(
            name(),
            "Cannot extract argument as string".to_owned(),
        ))?);
        let path = if path.is_absolute() {
            path
        } else if let Some(cwd) = sc.get_cwd().await? {
            cwd.join(path)
        } else {
            return Err(E::Executing(
                name(),
                format!(
                    "Cannot switch to relative \"{}\" because CWD isn't setup",
                    path.to_string_lossy()
                ),
            ));
        };
        if !path.exists() {
            return Err(E::Executing(
                name(),
                format!("Folder {} doesn't exist", path.to_string_lossy()),
            ));
        }
        sc.set_cwd(Some(path.clone())).await?;
        Ok(AnyValue::new(path)?)
    })
}
