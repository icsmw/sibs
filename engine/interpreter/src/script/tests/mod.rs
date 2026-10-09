mod modules;
mod warnings;

use super::*;

#[test]
fn prepares_context_for_valid_script() {
    for options in [ScriptOptions::strict(), ScriptOptions::resilient()] {
        let mut ctx = InterContext::default();
        Script::from_text(
            "component comp() { task run() { true; } };",
            options,
            &mut ctx,
        )
        .unwrap();

        assert!(ctx
            .get_anchor_inner()
            .unwrap()
            .get_component("comp")
            .is_some());
        assert!(ctx.get_semantic_cx().is_some());
        assert!(!ctx.get_diagnostics().unwrap().has_errors());
    }
}

#[test]
fn rejects_reuse_of_prepared_context_without_changing_it() {
    let content = "component original() { task run() { true; } };";
    let mut ctx = InterContext::default();
    Script::from_text(content, ScriptOptions::strict(), &mut ctx).unwrap();
    let source = *ctx.get_diagnostics().unwrap().tokens().root();
    let anchor = ctx.get_anchor_inner().unwrap().uuid;

    for replacement in [
        "component replacement() { task run() { false; } };",
        "component replacement() { task run() { let value = ; } };",
    ] {
        let result = Script::from_text(replacement, ScriptOptions::strict(), &mut ctx);
        assert!(
            matches!(result, Err(ScriptError::UsedContext)),
            "{result:?}"
        );
        assert_eq!(ctx.get_anchor_inner().unwrap().uuid, anchor);
        assert!(ctx.get_semantic_cx().is_some());
        let diagnostics = ctx.get_diagnostics().unwrap();
        assert_eq!(*diagnostics.tokens().root(), source);
        assert!(!diagnostics.has_errors());
        assert_eq!(
            diagnostics
                .sources()
                .get_content(&source)
                .unwrap()
                .as_deref(),
            Some(content)
        );
    }
}

#[test]
fn rejects_reuse_of_context_with_only_diagnostics() {
    for content in [
        "",
        "// only a comment",
        "component comp() { task run() { let value = ; } };",
    ] {
        let mut ctx = InterContext::default();
        assert!(Script::from_text(content, ScriptOptions::strict(), &mut ctx).is_err());
        assert!(ctx.get_anchor().is_none());
        assert!(ctx.get_semantic_cx().is_none());
        let diagnostics = ctx.get_diagnostics().unwrap();
        let source = *diagnostics.tokens().root();
        let error_count = diagnostics.errors().len();

        let result = Script::from_text(
            "component replacement() { task run() { true; } };",
            ScriptOptions::strict(),
            &mut ctx,
        );
        assert!(
            matches!(result, Err(ScriptError::UsedContext)),
            "{result:?}"
        );
        assert!(ctx.get_anchor().is_none());
        assert!(ctx.get_semantic_cx().is_none());
        let diagnostics = ctx.get_diagnostics().unwrap();
        assert_eq!(*diagnostics.tokens().root(), source);
        assert_eq!(diagnostics.errors().len(), error_count);
        assert_eq!(
            diagnostics
                .sources()
                .get_content(&source)
                .unwrap()
                .as_deref(),
            Some(content)
        );
    }
}

#[test]
fn resilient_collects_semantic_errors_and_preserves_analysis() {
    let mut ctx = InterContext::default();
    let content =
        "component comp() { task run() { let first: num = true; let second: bool = 5; } };";
    let result = Script::from_text(content, ScriptOptions::resilient(), &mut ctx);
    assert!(
        matches!(result, Err(ScriptError::NotExecutable)),
        "{result:?}"
    );

    let diagnostics = ctx.get_diagnostics().unwrap();
    let errors = diagnostics
        .errors()
        .iter()
        .filter(|err| err.severity == diagnostics::Severity::Error)
        .collect::<Vec<_>>();
    assert_eq!(errors.len(), 2, "{errors:?}");
    assert!(errors
        .iter()
        .all(|err| matches!(err.e, DiagnosticError::Semantic(_))));
    assert!(errors.iter().all(|err| err.link.from.abs < err.link.to.abs));
    assert_ne!(errors[0].link.from.abs, errors[1].link.from.abs);
    assert_eq!(
        diagnostics
            .sources()
            .get_content(&errors[0].link.src)
            .unwrap()
            .as_deref(),
        Some(content)
    );
    assert!(ctx
        .get_anchor_inner()
        .unwrap()
        .get_component("comp")
        .is_some());
    assert!(ctx.get_semantic_cx().unwrap().errs.is_empty());
}

#[test]
fn recovered_parsing_errors_make_script_not_executable() {
    let mut ctx = InterContext::default();
    let content = "component comp() { task run() { let value = ; } };";
    let result = Script::from_text(content, ScriptOptions::resilient(), &mut ctx);

    assert!(
        matches!(result, Err(ScriptError::NotExecutable)),
        "{result:?}"
    );
    assert!(ctx.get_anchor().is_some());
    assert!(ctx.get_semantic_cx().is_some());
    let errors = ctx
        .get_diagnostics()
        .unwrap()
        .errors()
        .iter()
        .filter(|err| err.severity == diagnostics::Severity::Error)
        .collect::<Vec<_>>();
    assert!(!errors.is_empty());
    assert!(errors
        .iter()
        .all(|err| matches!(err.e, DiagnosticError::Parser(_))));
}

#[test]
fn strict_stops_at_first_semantic_error_and_keeps_diagnostics() {
    let mut ctx = InterContext::default();
    let content =
        "component comp() { task run() { let first: num = true; let second: bool = 5; } };";
    let result = Script::from_text(content, ScriptOptions::strict(), &mut ctx);

    assert!(
        matches!(result, Err(ScriptError::NotExecutable)),
        "{result:?}"
    );
    let errors = ctx
        .get_diagnostics()
        .unwrap()
        .errors()
        .iter()
        .filter(|err| err.severity == diagnostics::Severity::Error)
        .collect::<Vec<_>>();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(matches!(errors[0].e, DiagnosticError::Semantic(_)));
    assert!(errors[0].link.to.abs <= content.find("let second").unwrap());
    assert!(ctx.get_anchor().is_some());
    assert!(ctx.get_semantic_cx().is_some());
}

#[test]
fn parsing_failure_keeps_diagnostics_without_anchor() {
    let mut ctx = InterContext::default();
    let content = "component comp() { task run() { let value = ; } };";
    let result = Script::from_text(content, ScriptOptions::strict(), &mut ctx);

    assert!(
        matches!(result, Err(ScriptError::NotExecutable)),
        "{result:?}"
    );
    assert!(ctx.get_anchor().is_none());
    assert!(ctx.get_semantic_cx().is_none());
    let diagnostics = ctx.get_diagnostics().unwrap();
    let errors = diagnostics
        .errors()
        .iter()
        .filter(|err| err.severity == diagnostics::Severity::Error)
        .collect::<Vec<_>>();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0].e, DiagnosticError::Parser(_)));
    assert_eq!(
        diagnostics
            .sources()
            .get_content(&errors[0].link.src)
            .unwrap()
            .as_deref(),
        Some(content)
    );
}
