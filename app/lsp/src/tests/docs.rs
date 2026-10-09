use crate::*;

#[tokio::test]
async fn documentation_warnings_have_warning_severity_and_disappear_after_editing() {
    let (service, _socket) = LspService::new(Backend::new);
    let backend = service.inner();
    let warnings = backend
        .get_diagnostics("component comp() { task run() { true; } };")
        .await
        .unwrap();
    assert_eq!(warnings.len(), 2);
    assert!(warnings
        .iter()
        .all(|diag| diag.severity == Some(DiagnosticSeverity::WARNING)));
    assert!(warnings.iter().all(|diag| matches!(
        &diag.code,
        Some(NumberOrString::String(code)) if code.starts_with("LI-")
    )));
    assert!(warnings
        .iter()
        .all(|diag| diag.code.is_some() && diag.range.start < diag.range.end));
    let diagnostics = backend
        .get_diagnostics(
            "/// Component docs.\ncomponent comp() {\n/// Task docs.\ntask run() { true; } };",
        )
        .await
        .unwrap();
    assert!(diagnostics.is_empty());

    let diagnostics = backend
        .get_diagnostics("component comp() { task run() { missing(); } };")
        .await
        .unwrap();
    assert!(diagnostics
        .iter()
        .any(|diag| diag.severity == Some(DiagnosticSeverity::ERROR)));
    assert!(diagnostics
        .iter()
        .any(|diag| diag.severity == Some(DiagnosticSeverity::WARNING)));
}
