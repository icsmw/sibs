use crate::{
    executors::{get_last_name, ExecutorPinnedResult},
    inf::{AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_last_name(module_path!())
}

pub fn execute(args: Vec<AnyValue>, cx: Context, _sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        args.iter().for_each(|arg| {
            if let Some(str) = arg.get_as_string() {
                println!("{str}");
            } else {
                println!("{arg:?}");
            }
        });
        Ok(AnyValue::new(()))
    })
}

#[cfg(test)]
mod test {
    use std::process::Command;

    #[test]
    fn printing() {
        let cwd = std::env::current_dir().expect("current folder detected");
        let output = Command::new(if cfg!(target_os = "windows") {
            "sibs.exe"
        } else {
            "./sibs"
        })
        .current_dir(cwd.join("../target/debug"))
        .args([
            "--scenario",
            cwd.join("./src/tests/cli/print.sibs")
                .to_str()
                .expect("path parsed"),
            "a",
            "success",
            "--output",
            "logs",
        ])
        .output()
        .expect("failed to execute process");
        assert_eq!(output.status.code(), Some(0));
        assert!(String::from_utf8_lossy(&output.stdout).contains("Hello World!"));
    }
}
