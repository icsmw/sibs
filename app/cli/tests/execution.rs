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
    task echo(message: str) { print(message); };
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
