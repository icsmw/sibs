use super::*;
use diagnostics::Severity;
use lint::LintError;

#[test]
fn lint_checks_a_parsed_tree_without_semantic_initialization() {
    let content = "component comp() { task run() { missing(); } };";
    let mut lexer = lexer::Lexer::new(content, 0);
    let parser = Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, content, false);
    let root = LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Anchor]))
        .unwrap()
        .unwrap();
    let diagnostics = lint::check(&root);
    assert_eq!(diagnostics.slice().len(), 2);
    assert!(!diagnostics.has_errors());
    assert!(diagnostics
        .slice()
        .iter()
        .all(|err| err.e.src() == diagnostics::ErrorSource::Lint));

    // Semantic analysis still reports the unknown function, but owns no doc checks.
    let mut scx = SemanticCx::new(false);
    let result = root
        .initialize(&mut scx)
        .and_then(|_| root.infer_type(&mut scx).map(|_| ()))
        .and_then(|_| root.finalize(&mut scx));
    assert!(matches!(
        result,
        Err(LinkedErr {
            e: SemanticError::FnNotFound(_),
            ..
        })
    ));
    assert!(scx.errs.is_empty());
}

#[test]
fn lint_reaches_later_components_even_when_strict_semantics_stops_early() {
    let mut ctx = InterContext::default();
    let result = Script::from_text(
        "component first() { task run() { missing(); } }; component second() { task run() { true; } };",
        ScriptOptions::strict(),
        &mut ctx,
    );
    assert!(matches!(result, Err(ScriptError::NotExecutable)));
    let diagnostics = ctx.get_diagnostics().unwrap();
    assert!(diagnostics.has_errors());
    let warnings = diagnostics
        .errors()
        .iter()
        .filter(|err| err.severity == Severity::Warning)
        .collect::<Vec<_>>();
    assert_eq!(warnings.len(), 4, "{warnings:?}");
    assert!(warnings.iter().any(|err| matches!(&err.e,
        DiagnosticError::Lint(LintError::MissingComponentDocs(name)) if name == "second")));
}

#[test]
fn missing_docs_are_warnings_in_both_modes_and_keep_source_positions() {
    let content = "//! Scenario docs do not document the component.\ncomponent comp() { private task run(value: str) { true; } };";
    for options in [ScriptOptions::strict(), ScriptOptions::resilient()] {
        let mut ctx = InterContext::default();
        Script::from_text(content, options, &mut ctx).unwrap();
        let diagnostics = ctx.get_diagnostics().unwrap();
        assert!(!diagnostics.has_errors());
        assert!(diagnostics.has_warnings());
        let warnings = diagnostics.errors();
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        assert!(warnings.iter().all(|err| err.severity == Severity::Warning));
        assert!(matches!(&warnings[0].e,
            DiagnosticError::Lint(LintError::MissingComponentDocs(name)) if name == "comp"));
        assert!(matches!(&warnings[1].e,
            DiagnosticError::Lint(LintError::MissingTaskDocs(name)) if name == "run"));
        for (warning, name) in warnings.iter().zip(["comp", "run"]) {
            assert_eq!(&content[warning.link.from.abs..warning.link.to.abs], name);
            assert_eq!(&warning.link.src, diagnostics.tokens().root());
        }
        assert!(ctx.get_semantic_cx().unwrap().errs.is_empty());
    }
}

#[test]
fn task_argument_documentation_can_be_local_or_in_arguments_section() {
    let content = r#"/// Component docs.
component comp() {
    /// Task docs.
    ///
    /// # Arguments
    /// * `first` - First argument.
    /// * `empty` -
    /// * `unknown` - This does not document `missing`.
    task run(
        first: str,
        /// Second argument.
        second: str,
        empty: str,
        missing: str
    ) { true; };
};"#;
    for options in [ScriptOptions::strict(), ScriptOptions::resilient()] {
        let mut ctx = InterContext::default();
        Script::from_text(content, options, &mut ctx).unwrap();
        let warnings = ctx.get_diagnostics().unwrap().errors();
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        for (warning, name) in warnings.iter().zip(["empty", "missing"]) {
            assert_eq!(warning.severity, Severity::Warning);
            assert!(matches!(&warning.e,
                DiagnosticError::Lint(LintError::MissingTaskArgumentDocs(task, arg))
                if task == "run" && arg == name));
            assert_eq!(&content[warning.link.from.abs..warning.link.to.abs], name);
        }
    }
}

#[test]
fn empty_docs_and_missing_arguments_section_are_reported() {
    let content = r#"// An ordinary comment is not documentation.
///
component comp() {
    ///
    task empty() { true; };
    /// Describes the task, but not its argument.
    task partial(value: str) { true; };
    /// Fully documented task without arguments.
    task complete() { true; };
};"#;
    let mut ctx = InterContext::default();
    Script::from_text(content, ScriptOptions::strict(), &mut ctx).unwrap();
    let warnings = ctx.get_diagnostics().unwrap().errors();
    assert_eq!(warnings.len(), 3, "{warnings:?}");
    assert!(matches!(&warnings[2].e,
        DiagnosticError::Lint(LintError::MissingTaskArgumentDocs(task, arg))
        if task == "partial" && arg == "value"));
}

#[test]
fn complete_documentation_emits_no_warnings() {
    let content = r#"/// Component docs.
component comp() {
    /// Task docs.
    ///
    /// # Arguments
    /// * `first` - First argument.
    task run(
        first: str,
        /// Second argument.
        second: str
    ) { true; };
};"#;
    let mut ctx = InterContext::default();
    Script::from_text(content, ScriptOptions::strict(), &mut ctx).unwrap();
    assert!(ctx.get_diagnostics().unwrap().errors().is_empty());
}

#[test]
fn argument_docs_respect_markdown_item_boundaries_and_code_indentation() {
    let content = r#"/// Component docs.
component comp() {
    /// Task docs.
    ///
    /// # Arguments
    ///
    ///     * `code_example` - This is code, not an argument description.
    ///
    /// * `empty` -
    ///
    /// A separate paragraph, not a description of `empty`.
    ///
    /// * `complete`:
    ///
    ///   The description continues inside this list item.
    task run(code_example: str, empty: str, complete: str) { true; };
};"#;
    let mut ctx = InterContext::default();
    Script::from_text(content, ScriptOptions::strict(), &mut ctx).unwrap();
    let warnings = ctx.get_diagnostics().unwrap().errors();
    assert_eq!(warnings.len(), 2, "{warnings:?}");
    for (warning, expected) in warnings.iter().zip(["code_example", "empty"]) {
        assert!(matches!(&warning.e,
            DiagnosticError::Lint(LintError::MissingTaskArgumentDocs(_, name)) if name == expected));
    }
}

#[test]
fn warnings_survive_a_later_blocking_error() {
    for options in [ScriptOptions::strict(), ScriptOptions::resilient()] {
        let mut ctx = InterContext::default();
        let result = Script::from_text(
            "component comp() { task run() { let value: num = true; } };",
            options,
            &mut ctx,
        );
        assert!(matches!(result, Err(ScriptError::NotExecutable)));
        let diagnostics = ctx.get_diagnostics().unwrap();
        assert!(diagnostics.has_errors());
        assert_eq!(
            diagnostics
                .errors()
                .iter()
                .filter(|err| err.severity == Severity::Warning)
                .count(),
            2
        );
        assert!(ctx.get_semantic_cx().unwrap().errs.is_empty());
    }
}

#[test]
fn included_component_warnings_point_to_the_included_source() {
    let files = test_utils::Files::new();
    let included = "component comp() { task run() { true; } };";
    files.write("included.sibs", included);
    let mut ctx = InterContext::default();
    Script::from_file(
        files.write("main.sibs", "include from \"included.sibs\";"),
        ScriptOptions::strict(),
        &mut ctx,
    )
    .unwrap();
    let diagnostics = ctx.get_diagnostics().unwrap();
    assert_eq!(diagnostics.errors().len(), 2);
    for warning in diagnostics.errors() {
        assert_ne!(&warning.link.src, diagnostics.tokens().root());
        assert_eq!(
            diagnostics
                .sources()
                .get_content(&warning.link.src)
                .unwrap()
                .as_deref(),
            Some(included)
        );
    }
}
