use crate::{
    elements::FuncArg,
    functions::ExecutorPinnedResult,
    inf::{tools::get_last_name, Context, Scope, Value},
};

pub fn name() -> String {
    get_last_name(module_path!())
}

pub fn execute(
    args: Vec<FuncArg>,
    _args_token: usize,
    _cx: Context,
    _sc: Scope,
) -> ExecutorPinnedResult {
    Box::pin(async move {
        args.iter().for_each(|arg| {
            if let Some(str) = arg.value.as_string() {
                println!("{str}");
            } else {
                println!("{arg:?}");
            }
        });
        Ok(Value::empty())
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
            "print",
            "--output",
            "logs",
        ])
        .output()
        .expect("failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("STDOUT:\n{stdout}\n");
        println!("STDOUT:\n{stderr}\n");
        assert_eq!(output.status.code(), Some(0));
        assert!(stdout.contains("Hello World!"));
    }
}
