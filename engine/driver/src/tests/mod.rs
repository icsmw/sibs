mod bindings;

use super::*;

#[test]
fn root_metadata_is_preserved_and_highlighted() {
    let mut driver = Driver::unbound(
        "//! Script docs\n/// Component docs\ncomponent comp() { task run() { true; } };",
        false,
    );
    driver.read().unwrap();
    let root = driver.ctx.get_anchor().unwrap();
    assert_eq!(root.get_md().lines(), ["Script docs"]);
    assert_eq!(
        root.extract::<Anchor>()
            .unwrap()
            .get_component("comp")
            .unwrap()
            .get_md()
            .lines(),
        ["Component docs"]
    );
    let tokens = driver.get_semantic_tokens();
    assert!(tokens.iter().any(|tk| tk.position.from.abs == 0));
}

#[test]
fn reads_file_sources_and_semantic_tokens() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/playground/test.sibs");
    let content = std::fs::read_to_string(&path).unwrap();
    let mut driver = Driver::new(path, true);
    driver.read().unwrap();

    assert_eq!(
        driver.get_src_content(None).unwrap().as_deref(),
        Some(content.as_str())
    );
    let tokens = driver.get_semantic_tokens();
    assert!(!tokens.is_empty());
    for token in tokens {
        assert!(token.extract_by_relative(&content).is_some());
    }
}

#[test]
fn preserves_source_when_no_anchor_is_found() {
    for content in ["", "// only a comment"] {
        for resilience in [false, true] {
            let mut driver = Driver::unbound(content, resilience);
            let result = driver.read();
            if resilience {
                result.unwrap();
            } else {
                assert!(
                    matches!(result, Err(E::FailExtractAnchorNodeFrom(_))),
                    "{result:?}"
                );
            }
            assert!(driver.ctx.get_anchor().is_none());
            assert!(driver.ctx.get_semantic_cx().is_none());
            assert!(driver.errors().unwrap().next().is_none());
            assert_eq!(
                driver.get_src_content(None).unwrap().as_deref(),
                Some(content)
            );
        }
    }
}

#[test]
fn reading_again_replaces_sources_and_errors() {
    let mut driver = Driver::unbound(
        "component comp() { task run() { let value: num = true; } };",
        true,
    );
    driver.read().unwrap();
    assert!(driver.errors().unwrap().next().is_some());
    let old_src = driver
        .ctx
        .get_diagnostics()
        .unwrap()
        .get_token(0)
        .unwrap()
        .src;

    let content = "component next() { task run() { true; } };";
    driver.src = CodeSrc::Text(content.to_owned());
    driver.read().unwrap();
    assert_eq!(driver.errors().unwrap().count(), 0);
    assert!(driver
        .ctx
        .get_anchor_inner()
        .unwrap()
        .get_component("comp")
        .is_none());
    assert!(driver
        .ctx
        .get_anchor_inner()
        .unwrap()
        .get_component("next")
        .is_some());
    assert_eq!(
        driver.get_src_content(None).unwrap().as_deref(),
        Some(content)
    );
    assert!(driver.get_src_content(Some(&old_src)).unwrap().is_none());
}
