use super::*;

#[test]
fn resilient_analysis_reports_unknown_function_once() {
    let mut driver = Driver::unbound("component comp() { task run() { missing(); } };", true);
    driver.read().unwrap();

    let mut errors = driver.errors().unwrap();
    let error = errors.next().expect("unknown function is reported");
    assert!(matches!(
        &error.err.e,
        DiagnosticError::Semantic(SemanticError::FnNotFound(name)) if name == "missing"
    ));
    assert!(errors.next().is_none(), "duplicate diagnostics remain");
}

#[test]
fn resilient_analysis_reports_unknown_variable_once() {
    let mut driver = Driver::unbound(
        "component comp() { task run() { let x = unknown; } };",
        true,
    );
    driver.read().unwrap();

    let mut errors = driver.errors().unwrap();
    let error = errors.next().expect("unknown variable is reported");
    assert!(matches!(
        &error.err.e,
        DiagnosticError::Semantic(SemanticError::VariableIsNotDefined(name)) if name == "unknown"
    ));
    assert!(errors.next().is_none(), "duplicate diagnostics remain");
}

#[test]
fn exposes_parser_diagnostics_without_anchor() {
    let content = "component comp() { task run() { let value = ; } };";
    let mut driver = Driver::unbound(content, false);
    assert!(matches!(driver.read(), Err(E::NotExecutable)));

    assert!(driver.ctx.get_anchor().is_none());
    let errors = driver
        .errors()
        .expect("diagnostics exist without an AST")
        .collect::<Vec<_>>();
    assert!(!errors.is_empty());
    for error in errors {
        assert!(matches!(error.err.e, DiagnosticError::Parser(_)));
        assert!(!error.err.to_string().is_empty());
        assert!(error
            .locator
            .get_ownership_tree(error.err.link.from.abs)
            .is_empty());
        let mut output = Vec::new();
        error
            .err
            .report(driver.ctx.get_diagnostics().unwrap(), &mut output)
            .unwrap();
        assert!(String::from_utf8(output).unwrap().contains(content));
    }
}

#[test]
fn resilient_parsing_preserves_ast_and_reports_errors() {
    let mut driver = Driver::unbound("component comp() { task run() { let value = ; } };", true);
    driver.read().unwrap();

    assert!(driver.ctx.get_anchor().is_some());
    assert!(driver
        .errors()
        .unwrap()
        .any(|error| matches!(error.err.e, DiagnosticError::Parser(_))));
}

#[test]
fn exposes_semantic_diagnostics_after_strict_failure() {
    let content = "component comp() { task run() { let value: num = true; } };";
    let mut driver = Driver::unbound(content, false);
    assert!(matches!(driver.read(), Err(E::NotExecutable)));

    let errors = driver.errors().unwrap().collect::<Vec<_>>();
    assert_eq!(errors.len(), 1);
    let error = &errors[0];
    assert!(matches!(error.err.e, DiagnosticError::Semantic(_)));
    assert!(error.err.link.from.abs < error.err.link.to.abs);
    assert!(!error
        .locator
        .get_ownership_tree(error.err.link.from.abs)
        .is_empty());
    assert_eq!(
        driver
            .get_src_content(Some(&error.err.link.src))
            .unwrap()
            .as_deref(),
        Some(content)
    );
}
