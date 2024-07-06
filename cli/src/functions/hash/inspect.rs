use crate::{
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(args: Vec<AnyValue>, _cx: Context, sc: Scope) -> ExecutorPinnedResult {
    Box::pin(async move {
        let cwd = sc
            .get_cwd()
            .await?
            .ok_or(E::IO(String::from("No CWD path")))?;
        let mut paths = Vec::new();
        for arg in args.iter() {
            let Some(arg) = arg.as_path_buf() else {
                return Err(E::InvalidFunctionArg(format!(
                    "{arg:?} has to be a string or path"
                )));
            };
            paths.push(cwd.join(arg));
        }
        Ok(AnyValue::bool(true))
    })
}

// #[cfg(test)]
// mod test {
//     use std::process::Command;

//     #[test]
//     fn printing() {
//         let cwd = std::env::current_dir().expect("current folder detected");
//         let output = Command::new(if cfg!(target_os = "windows") {
//             "sibs.exe"
//         } else {
//             "./sibs"
//         })
//         .current_dir(cwd.join("../target/debug"))
//         .args([
//             "--scenario",
//             cwd.join("./src/tests/cli/print.sibs")
//                 .to_str()
//                 .expect("path parsed"),
//             "a",
//             "print",
//             "--output",
//             "logs",
//         ])
//         .output()
//         .expect("failed to execute process");
//         assert_eq!(output.status.code(), Some(0));
//         assert!(String::from_utf8_lossy(&output.stdout).contains("Hello World!"));
//     }
// }
