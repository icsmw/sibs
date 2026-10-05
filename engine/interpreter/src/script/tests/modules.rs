use super::*;
use std::{path::PathBuf, sync::Arc};

struct Files(PathBuf);

impl Files {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!("sibs-module-script-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }

    fn write(&self, name: &str, content: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();
        path
    }

    fn prepare(&self, source: &str) -> InterContext {
        let mut ctx = InterContext::default();
        let result = Script::from_file(
            self.write("main.sibs", source),
            ScriptOptions::strict(),
            &mut ctx,
        );
        assert!(
            result.is_ok(),
            "{result:?}: {:?}",
            ctx.get_diagnostics().map(|d| d.errors())
        );
        ctx
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn repeated_imports_share_functions_scopes_and_closures() {
    let files = Files::new();
    files.write("shared.sibs", "fn helper(n: num) { n + 1; }; fn run(n: num) { let cb = |v: num| { helper(v); }; cb(n); helper(n); };");
    files.write("left.sibs", "mod from \"shared.sibs\";");
    files.write("right.sibs", "mod from \"shared.sibs\";");
    let mut ctx = files.prepare("mod from \"left.sibs\"; mod from \"right.sibs\"; component comp() { task run() { left::shared::run(5) + right::shared::run(6); } };");
    let anchor = ctx.get_anchor_inner().unwrap();
    let left = anchor.nodes[0].extract::<ModuleDeclaration>().unwrap();
    let right = anchor.nodes[1].extract::<ModuleDeclaration>().unwrap();
    let shared_left = left.body.nodes[0].extract::<ModuleDeclaration>().unwrap();
    let shared_right = right.body.nodes[0].extract::<ModuleDeclaration>().unwrap();
    assert_ne!(shared_left.uuid, shared_right.uuid);
    assert!(Arc::ptr_eq(&shared_left.body, &shared_right.body));
    let scx = ctx.get_semantic_cx().unwrap();
    assert_eq!(scx.fns.ufns.get_funcs().len(), 2);
    assert_eq!(scx.fns.cfns.get_funcs().len(), 1);
    for name in ["helper", "run"] {
        let left = scx.fns.find(format!("left::shared::{name}")).unwrap();
        let right = scx.fns.find(format!("right::shared::{name}")).unwrap();
        assert_eq!(left.uuid(), right.uuid());
    }
    assert!(scx.tys.get_scope(&shared_left.body.source).is_some());
    assert!(scx.tys.get_scope(&shared_left.uuid).is_none());
    assert!(scx.tys.get_scope(&shared_right.uuid).is_none());
    let value = Executor::new(ExecutionOptions::new("comp", "run", &files.0))
        .run(&mut ctx)
        .await
        .unwrap();
    assert_eq!(value, RtValue::Num(13.0));
}

#[test]
fn repeated_parent_imports_expose_nested_module_paths() {
    let files = Files::new();
    files.write("shared.sibs", "fn helper() { 12; };");
    files.write(
        "library.sibs",
        "mod nested { fn helper() { 7; }; }; mod from \"shared.sibs\";",
    );
    let ctx = files.prepare("mod left { mod from \"library.sibs\"; }; mod right { mod from \"library.sibs\"; }; component comp() { task run() { left::library::nested::helper() + right::library::shared::helper(); } };");
    let scx = ctx.get_semantic_cx().unwrap();
    assert_eq!(scx.fns.ufns.get_funcs().len(), 2);
    for suffix in ["nested::helper", "shared::helper"] {
        assert_eq!(
            scx.fns
                .find(format!("left::library::{suffix}"))
                .unwrap()
                .uuid(),
            scx.fns
                .find(format!("right::library::{suffix}"))
                .unwrap()
                .uuid()
        );
    }
}

#[test]
fn aliases_still_reject_a_different_function_at_the_same_path() {
    let files = Files::new();
    files.write("library.sibs", "fn helper() { 12; }; fn other() { 13; };");
    let source = "mod left { mod from \"library.sibs\"; }; mod right { mod library { fn helper() { 7; }; }; mod from \"library.sibs\"; };";
    let mut ctx = InterContext::default();
    assert!(Script::from_file(
        files.write("main.sibs", source),
        ScriptOptions::strict(),
        &mut ctx
    )
    .is_err());
    assert!(ctx.get_diagnostics().unwrap().errors().iter().any(|err|
        matches!(&err.e, DiagnosticError::Semantic(semantic::SemanticError::FnDeclarationError(message)) if message.contains("right::library::helper"))
    ));
    assert!(ctx
        .get_semantic_cx()
        .unwrap()
        .fns
        .find("right::library::other")
        .is_none());
}

#[test]
fn resilient_alias_conflicts_preserve_other_functions_and_module_paths() {
    let files = Files::new();
    files.write("shared.sibs", "fn helper() { 1; }; fn other() { 2; };");
    files.write(
        "library.sibs",
        r#"
        fn helper() { 3; };
        fn other() { 4; };
        mod nested { fn helper() { 5; }; fn other() { 6; }; };
        mod from "shared.sibs";
        fn after() { 7; };
        "#,
    );
    let source = r#"
        mod left { mod from "library.sibs"; };
        mod right {
            mod library {
                fn helper() { 8; };
                mod nested { fn helper() { 9; }; };
                mod shared { fn helper() { 10; }; };
            };
            mod from "library.sibs";
            fn outside() { 11; };
        };
        component comp() {
            task run() {
                right::library::other() + right::library::nested::other()
                    + right::library::shared::other() + right::library::after()
                    + right::outside();
            }
        };
    "#;
    let mut ctx = InterContext::default();
    let result = Script::from_file(
        files.write("main.sibs", source),
        ScriptOptions::resilient(),
        &mut ctx,
    );
    assert!(
        matches!(result, Err(ScriptError::NotExecutable)),
        "{result:?}"
    );
    let errors = ctx.get_diagnostics().unwrap().errors();
    assert_eq!(errors.len(), 3, "{errors:?}");
    for suffix in ["helper", "nested::helper", "shared::helper"] {
        let path = format!("right::library::{suffix}");
        assert!(errors.iter().any(|err|
            matches!(&err.e, DiagnosticError::Semantic(semantic::SemanticError::FnDeclarationError(message)) if message.contains(&path))
        ), "{errors:?}");
    }
    let scx = ctx.get_semantic_cx().unwrap();
    for suffix in ["other", "nested::other", "shared::other", "after"] {
        assert_eq!(
            scx.fns
                .find(format!("left::library::{suffix}"))
                .unwrap()
                .uuid(),
            scx.fns
                .find(format!("right::library::{suffix}"))
                .unwrap()
                .uuid(),
        );
    }
    assert!(scx.fns.find("right::outside").is_some());
}

#[test]
fn includes_keep_the_common_component_namespace() {
    let files = Files::new();
    files.write(
        "component.sibs",
        "component comp() { task run() { true; } };",
    );
    let mut ctx = InterContext::default();
    assert!(Script::from_file(
        files.write(
            "main.sibs",
            "include from \"component.sibs\"; include from \"./component.sibs\";"
        ),
        ScriptOptions::strict(),
        &mut ctx,
    )
    .is_err());
    let errors = ctx.get_diagnostics().unwrap().errors();
    assert!(
        errors.iter().any(|err| matches!(
            &err.e,
            DiagnosticError::Semantic(semantic::SemanticError::RtError(
                runtime::RtError::TaskDuplicate
            ))
        )),
        "{errors:?}"
    );
}

#[test]
fn repeated_empty_imports_share_a_body_and_keep_import_identities() {
    let files = Files::new();
    files.write("empty.sibs", "");
    let ctx = files.prepare("mod from \"empty.sibs\"; mod from \"./empty.sibs\"; component comp() { task run() { true; } };");
    let anchor = ctx.get_anchor_inner().unwrap();
    let first = anchor.nodes[0].extract::<ModuleDeclaration>().unwrap();
    let second = anchor.nodes[1].extract::<ModuleDeclaration>().unwrap();
    assert_ne!(first.uuid, second.uuid);
    assert!(Arc::ptr_eq(&first.body, &second.body));
    assert!(first.body.nodes.is_empty());
}

#[test]
fn import_cycles_are_reported_at_the_nested_path() {
    let files = Files::new();
    files.write("a.sibs", "mod from \"b.sibs\";");
    files.write("b.sibs", "mod from \"a.sibs\";");
    let mut ctx = InterContext::default();
    assert!(Script::from_file(
        files.write("main.sibs", "mod from \"a.sibs\";"),
        ScriptOptions::strict(),
        &mut ctx
    )
    .is_err());
    let diagnostics = ctx.get_diagnostics().unwrap();
    let error = diagnostics.errors().iter().find(|err| matches!(&err.e,
        DiagnosticError::Parser(ParserError::MissedExpectation(_, expected)) if expected == "acyclic file imports"
    )).expect("Import cycle is reported");
    assert_eq!(
        diagnostics
            .sources()
            .get_content(&error.link.src)
            .unwrap()
            .as_deref(),
        Some("mod from \"a.sibs\";")
    );
}

#[cfg(unix)]
#[test]
fn symlink_imports_share_a_body_and_resolve_dependencies_from_its_directory() {
    let files = Files::new();
    let library = files.write("real/library.sibs", "mod from \"dependency.sibs\";");
    files.write("real/dependency.sibs", "fn helper() { 12; };");
    std::os::unix::fs::symlink(library, files.0.join("alias.sibs")).unwrap();
    let ctx = files.prepare("mod from \"alias.sibs\"; mod from \"real/library.sibs\"; component comp() { task run() { alias::dependency::helper() + library::dependency::helper(); } };");
    let anchor = ctx.get_anchor_inner().unwrap();
    let first = anchor.nodes[0].extract::<ModuleDeclaration>().unwrap();
    let second = anchor.nodes[1].extract::<ModuleDeclaration>().unwrap();
    assert!(Arc::ptr_eq(&first.body, &second.body));
    assert_eq!(ctx.get_semantic_cx().unwrap().fns.ufns.get_funcs().len(), 1);
}
