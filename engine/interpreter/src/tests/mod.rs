mod efns;
mod macros;

mod cancellation;
mod panics;

mod globals;

// Change only the child process environment; tests in this process remain independent.
pub(crate) fn run_with_environment(
    name: &str,
    variables: &[(&str, Option<std::ffi::OsString>)],
) -> bool {
    if variables.is_empty() {
        return false;
    }
    let name = name.split_once("::").expect("Qualified test name").1;
    const CHILD: &str = "SIBS_ENV_TEST_CHILD";
    if std::env::var(CHILD).as_deref() == Ok(name) {
        return false;
    }
    let mut command = std::process::Command::new(std::env::current_exe().expect("Test executable"));
    command
        .args(["--exact", name, "--nocapture"])
        .env(CHILD, name);
    for (key, value) in variables {
        match value {
            Some(value) => {
                command.env(key, value);
            }
            None => {
                command.env_remove(key);
            }
        }
    }
    let result = command.output().expect("Child test process started");
    assert!(
        result.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stdout).contains("1 passed"),
        "Child must execute the requested test"
    );
    true
}
