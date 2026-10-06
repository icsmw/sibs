use super::*;

struct Files(PathBuf);

impl Files {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("sibs-driver-sources-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn write(&self, name: &str, content: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn source(driver: &Driver, content: &str) -> Uuid {
    *driver
        .tokens()
        .unwrap()
        .iter()
        .find(|(source, _)| {
            driver.get_src_content(Some(source)).unwrap().as_deref() == Some(content)
        })
        .unwrap()
        .0
}

#[test]
fn imported_sources_keep_tokens_owners_and_local_navigation() {
    let files = Files::new();
    let library = "fn helper(n: num) { n + 1; }; fn run(n: num) { helper(n); };";
    let component = "component comp() { task run() { strs::repeat(\"fd\", 12); } };";
    let values = "const limit: num = 3;";
    let envs = "optional SIBS_DRIVER_TOKEN_TEST;";
    files.write("library.sibs", library);
    files.write("component.sibs", component);
    files.write("values.sibs", values);
    files.write("envs.sibs", envs);
    let main = r#"globals from "values.sibs"; envs from "envs.sibs";
        mod left { mod from "library.sibs"; };
        mod right { mod from "library.sibs"; };
        include from "component.sibs";"#;
    let mut driver = Driver::new(files.write("main.sibs", main), false);
    driver.read().unwrap();
    let store = driver.tokens().unwrap();
    assert_eq!(store.iter().count(), 5);
    assert_eq!(driver.get_src_content(None).unwrap().as_deref(), Some(main));
    assert_eq!(driver.find_token(0, None).unwrap().0.src, *store.root());

    for content in [main, library, component, values, envs] {
        let src = source(&driver, content);
        let tokens = store.get(&src).unwrap();
        let mut owned = 0;
        for (idx, token) in tokens.iter().enumerate() {
            assert_eq!(token.src, src);
            if let Some((owner, _)) = token.owner {
                if token.pos.from.abs == token.pos.to.abs {
                    continue;
                }
                let pos = token.pos.from.abs;
                assert_eq!(driver.find_token(pos, Some(src)).unwrap().1, idx);
                let node = driver.find_node(pos, Some(src)).unwrap_or_else(|| {
                    panic!("Owner {owner} not found for {token:?} in {content}")
                });
                assert_eq!(*node.uuid(), owner);
                owned += 1;
            }
        }
        assert!(owned > 0, "No token owners in {content}");
        let (token, idx) = driver.find_token(0, Some(src)).unwrap();
        let mut locator = driver.locator(idx, Some(src)).unwrap();
        assert_eq!(locator.nth_token(idx as isize).unwrap().src, src);
        assert_eq!(locator.nth_tokens(idx..=idx)[0].unwrap().src, src);
        assert_eq!(locator.next_token().unwrap().token, token);
        locator.drop();
        assert_eq!(locator.prev_token().unwrap().token.src, src);
        locator.drop();
        while let Some(step) = locator.next_token() {
            assert_eq!(step.token.src, src);
            if let Some((owner, _)) = step.token.owner {
                assert_eq!(*step.node.expect("Token owner is in the AST").uuid(), owner);
            }
        }
    }

    let (idx, token) = store
        .get(store.root())
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, token)| token.owner.is_some_and(|(owner, _)| &owner == store.root()))
        .expect("Separators between declarations belong to the root anchor");
    let mut locator = driver.locator(idx, None).unwrap();
    assert!(locator
        .find(store.root())
        .unwrap()
        .extract::<Anchor>()
        .is_some());
    assert_eq!(locator.get_ownership_tree(token.pos.from.abs).len(), 1);
    assert_eq!(locator.next_node().unwrap().node.uuid(), store.root());
    let mut locator = driver.locator(idx, None).unwrap();
    assert_eq!(locator.prev_node().unwrap().node.uuid(), store.root());

    let src = source(&driver, component);
    let signature = driver
        .signature(component.find("12").unwrap(), Some(src))
        .unwrap();
    assert_eq!(signature.name, "strs::repeat");
    assert_eq!(signature.active, Some(1));

    let src = source(&driver, library);
    let signature = driver
        .signature(library.rfind("helper(n)").unwrap() + 7, Some(src))
        .unwrap();
    assert_eq!(signature.name, "helper");
    assert_eq!(signature.active, Some(0));

    let unknown = Uuid::new_v4();
    assert!(driver.find_token(0, Some(unknown)).is_none());
    assert!(driver.find_node(0, Some(unknown)).is_none());
    assert!(driver.locator(0, Some(unknown)).is_none());
}

#[test]
fn completion_in_a_shared_module_uses_one_import_path_and_its_scope() {
    let files = Files::new();
    let library = "fn run() { let local = 12; local; };";
    files.write("library.sibs", library);
    let mut driver = Driver::new(
        files.write(
            "main.sibs",
            r#"
        mod left { mod from "library.sibs"; };
        mod right { mod from "library.sibs"; };
    "#,
        ),
        false,
    );
    driver.read().unwrap();
    let src = source(&driver, library);
    let pos = library.rfind("local").unwrap();
    let (_, idx) = driver.find_token(pos, Some(src)).unwrap();
    let mut locator = driver.locator(idx, Some(src)).unwrap();
    let location = Location::detect(&mut locator).unwrap().unwrap();
    assert_eq!(location.mods, ["left", "library"]);
    assert_eq!(*location.get_scx_uuid(), src);
    let result = driver
        .completion(pos + 2, Some(src))
        .unwrap()
        .suggest()
        .unwrap()
        .unwrap();
    assert!(result.suggestions.iter().any(|suggestion| {
        matches!(&suggestion.target, CompletionMatch::Variable(name, _) if name == "local")
    }));
}

#[test]
fn repeated_includes_keep_the_first_token_owners() {
    let files = Files::new();
    let included = "globals from \"values.sibs\";";
    files.write("included.sibs", included);
    files.write("values.sibs", "const value: num = 12;");
    let mut driver = Driver::new(
        files.write(
            "main.sibs",
            r#"
        include from "included.sibs"; include from "./included.sibs";
    "#,
        ),
        false,
    );
    driver.read().unwrap();
    let anchor = driver.ctx.get_anchor_inner().unwrap();
    let first = anchor.nodes[0].extract::<IncludeDeclaration>().unwrap();
    let second = anchor.nodes[1].extract::<IncludeDeclaration>().unwrap();
    let first_import = &first.root.extract::<Anchor>().unwrap().nodes[0];
    let second_import = &second.root.extract::<Anchor>().unwrap().nodes[0];
    assert_ne!(first_import.uuid(), second_import.uuid());
    let src = source(&driver, included);
    assert_eq!(driver.tokens().unwrap().iter().count(), 3);
    assert_eq!(
        driver.find_node(0, Some(src)).unwrap().uuid(),
        first_import.uuid()
    );
    let locator = driver.locator(0, Some(src)).unwrap();
    let tree = locator.get_ownership_tree(0);
    assert!(tree.iter().any(|node| node.uuid() == &first.uuid));
    assert!(!tree.iter().any(|node| node.uuid() == &second.uuid));
}

#[test]
fn imported_error_locators_use_source_local_indices_even_without_an_ast() {
    for content in [
        "fn broken() { let value = ; };",
        "fn broken() { let value: num = true; };",
    ] {
        let files = Files::new();
        files.write("broken.sibs", content);
        let mut driver = Driver::new(files.write("main.sibs", "mod from \"broken.sibs\";"), false);
        assert!(driver.read().is_err());
        let src = source(&driver, content);
        let mut error = driver
            .errors()
            .unwrap()
            .find(|error| error.err.link.src == src)
            .unwrap();
        let (token, idx) = driver
            .find_token(error.err.link.from.abs, Some(src))
            .unwrap();
        assert_eq!(error.locator.idx, idx as isize);
        let step = error.locator.next_token().unwrap();
        assert_eq!(step.token.src, src);
        assert_eq!(step.token.pos, token.pos);
    }
}
