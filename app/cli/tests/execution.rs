use std::{fs, process::Command};

#[test]
fn runs_and_reports_scripts_through_cli() {
    let root = std::env::temp_dir().join(format!("sibs-cli-{}", uuid::Uuid::new_v4()));
    let scenario_dir = root.join("scenario");
    fs::create_dir_all(&scenario_dir).unwrap();
    fs::write(
        scenario_dir.join("scenario.sibs"),
        r#"//! Script documentation
/// Component documentation
component demo() {
    /// Echo documentation
    ///
    /// # Arguments
    /// * `message` - Message to print.
    task echo(message: str) { print(message); };
    /// Creates a marker in the working directory.
    task cwd() { `touch cwd-marker`; };
};"#,
    )
    .unwrap();
    fs::write(
        scenario_dir.join("syntax.sibs"),
        "component demo() { task run() { let value = ; print(\"UNEXPECTED_EXECUTION\"); } };",
    )
    .unwrap();
    fs::write(
        scenario_dir.join("semantic.sibs"),
        "component demo() { task run() { let value: num = true; print(\"UNEXPECTED_EXECUTION\"); } };",
    )
    .unwrap();

    let cases: &[(&str, &str, &[&str], i32, &str)] = &[
        (
            "argument",
            "scenario.sibs",
            &["demo", "echo", "CLI_ARGUMENT_OK"],
            0,
            "CLI_ARGUMENT_OK",
        ),
        (
            "components",
            "scenario.sibs",
            &["--help"],
            0,
            "Component documentation",
        ),
        (
            "tasks",
            "scenario.sibs",
            &["--help", "demo"],
            0,
            "Echo documentation",
        ),
        ("syntax", "syntax.sibs", &["demo", "run"], 1, "^"),
        ("semantic", "semantic.sibs", &["demo", "run"], 1, "^"),
        (
            "execution",
            "scenario.sibs",
            &["missing", "echo"],
            1,
            "Component \"missing\"",
        ),
        ("cwd", "scenario.sibs", &["demo", "cwd"], 0, ""),
    ];
    for &(name, file, args, exit, expected) in cases {
        let path = scenario_dir.join(file);
        let output = Command::new(env!("CARGO_BIN_EXE_cli"))
            .current_dir(&root)
            .arg("--scenario")
            .arg(&path)
            .args(args)
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert_eq!(
            output.status.code(),
            Some(exit),
            "{name}: {stdout}\n{stderr}"
        );
        if exit == 0 {
            assert!(stdout.contains(expected), "{name}: {stdout}\n{stderr}");
            assert!(stderr.is_empty(), "{name}: {stderr}");
        } else {
            assert!(stderr.contains(expected), "{name}: {stderr}");
            assert!(stderr.contains(path.to_str().unwrap()), "{name}: {stderr}");
            assert!(!stdout.contains("UNEXPECTED_EXECUTION"), "{name}: {stdout}");
        }
    }
    assert!(scenario_dir.join("cwd-marker").is_file());
    assert!(!root.join("cwd-marker").exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn warnings_do_not_block_execution_or_help_and_stay_on_stderr() {
    let root = std::env::temp_dir().join(format!("sibs-cli-warnings-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&root).unwrap();
    let path = root.join("scenario.sibs");
    fs::write(
        &path,
        "component demo() { task echo(message: str) { print(message); } };",
    )
    .unwrap();
    for args in [
        vec!["demo", "echo", "EXECUTED_WITH_WARNINGS"],
        vec!["--help"],
        vec!["demo", "--help"],
        vec!["demo", "echo", "--help"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_cli"))
            .current_dir(&root)
            .arg("--scenario")
            .arg(&path)
            .args(&args)
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(output.status.success(), "{args:?}: {stdout}\n{stderr}");
        assert_eq!(stderr.matches("warning:").count(), 2, "{stderr}");
        assert!(stderr.contains("Component \"demo\" has no documentation"));
        assert!(stderr.contains("Task \"echo\" has no documentation"));
        assert!(!stdout.contains("warning:"));
        if args.contains(&"--help") {
            assert!(stdout.contains("echo"));
            assert!(!stdout.contains("EXECUTED_WITH_WARNINGS"));
        } else {
            assert!(stdout.contains("EXECUTED_WITH_WARNINGS"));
        }
    }
    fs::remove_dir_all(root).unwrap();
}
