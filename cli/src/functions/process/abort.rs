use crate::{
    elements::FuncArg,
    functions::ExecutorPinnedResult,
    inf::{operator, tools::get_last_name, Value, Context, Scope},
};

pub fn name() -> String {
    get_last_name(module_path!())
}

pub fn execute(
    args: Vec<FuncArg>,
    _args_token: usize,
    cx: Context,
    _sc: Scope,
) -> ExecutorPinnedResult {
    Box::pin(async move {
        cx.abort(
            if let Some(arg) = args.first() {
                arg.value.as_num().ok_or(operator::E::FailToExtractValue)?
            } else {
                0
            } as i32,
            if let Some(arg) = args.get(1) {
                Some(
                    arg.value
                        .as_string()
                        .ok_or(operator::E::FailToExtractValue)?,
                )
            } else {
                None
            },
        )
        .await?;
        Ok(Value::empty())
    })
}

#[cfg(test)]
mod test {
    use crate::inf::tests::*;
    use std::process::Command;

    #[test]
    fn success() {
        let cwd = std::env::current_dir().expect("current folder detected");
        let output = Command::new(if cfg!(target_os = "windows") {
            "sibs.exe"
        } else {
            "./sibs"
        })
        .current_dir(cwd.join("../target/debug"))
        .args([
            "--scenario",
            cwd.join("./src/tests/cli/abort.sibs")
                .to_str()
                .expect("path parsed"),
            "abort",
            "success_test",
            "--output",
            "logs",
        ])
        .output()
        .expect("failed to execute process");
        print_stdout(&output);
        assert_eq!(output.status.code(), Some(0));
    }

    #[test]
    fn success_with_message() {
        let cwd = std::env::current_dir().expect("current folder detected");
        let output = Command::new(if cfg!(target_os = "windows") {
            "sibs.exe"
        } else {
            "./sibs"
        })
        .current_dir(cwd.join("../target/debug"))
        .args([
            "--scenario",
            cwd.join("./src/tests/cli/abort.sibs")
                .to_str()
                .expect("path parsed"),
            "abort",
            "success_with_message_test",
            "--output",
            "logs",
        ])
        .output()
        .expect("failed to execute process");
        print_stdout(&output);
        assert_eq!(output.status.code(), Some(0));
        assert!(String::from_utf8_lossy(&output.stdout).contains("Hello World!"));
    }

    #[test]
    fn error() {
        let cwd = std::env::current_dir().expect("current folder detected");
        let output = Command::new(if cfg!(target_os = "windows") {
            "sibs.exe"
        } else {
            "./sibs"
        })
        .current_dir(cwd.join("../target/debug"))
        .args([
            "--scenario",
            cwd.join("./src/tests/cli/abort.sibs")
                .to_str()
                .expect("path parsed"),
            "abort",
            "error_test",
            "--output",
            "logs",
        ])
        .output()
        .expect("failed to execute process");
        print_stdout(&output);
        assert_eq!(output.status.code(), Some(1));
    }

    #[test]
    fn error_with_message() {
        let cwd = std::env::current_dir().expect("current folder detected");
        let output = Command::new(if cfg!(target_os = "windows") {
            "sibs.exe"
        } else {
            "./sibs"
        })
        .current_dir(cwd.join("../target/debug"))
        .args([
            "--scenario",
            cwd.join("./src/tests/cli/abort.sibs")
                .to_str()
                .expect("path parsed"),
            "abort",
            "error_with_message_test",
            "--output",
            "logs",
        ])
        .output()
        .expect("failed to execute process");
        print_stdout(&output);
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("Hello World!"));
    }
}
