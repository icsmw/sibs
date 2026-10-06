use super::*;
use test_utils::Files;

#[test]
fn sibling_imports_share_identity_but_not_ancestry() {
    let files = Files::new();
    let mut sources = CodeSources::default();
    let root = sources.enter_file(files.write("main", ""), None).unwrap();
    let left = sources
        .enter_file(files.write("left", ""), Some(&root))
        .unwrap();
    let right = sources
        .enter_file(files.write("right", ""), Some(&root))
        .unwrap();
    let common = files.write("common", "");
    let first = sources.enter_file(&common, Some(&left)).unwrap();
    let second = sources
        .enter_file(files.path().join("./common"), Some(&right))
        .unwrap();
    assert_eq!(first.source, second.source);
    assert_eq!(root.ancestry, vec![root.source]);
    assert_eq!(first.ancestry, vec![root.source, left.source, first.source]);
    assert_eq!(
        second.ancestry,
        vec![root.source, right.source, second.source]
    );
    assert!(matches!(
        sources.enter_file(&common, Some(&first)),
        Err(CodeSourceError::ImportCycle { .. })
    ));
    assert!(matches!(
        sources.enter_file(files.path().join("main"), Some(&second)),
        Err(CodeSourceError::ImportCycle { .. })
    ));
    assert_eq!(sources.sources.len(), 4);
}

#[test]
fn bound_source_preserves_identity_on_repeated_entry() {
    let files = Files::new();
    let path = files.write("source", "");
    let uuid = Uuid::new_v4();
    let mut sources = CodeSources::bound(&path, &uuid).unwrap();
    let root = sources.enter_file(&path, None).unwrap();
    assert_eq!(root.source, uuid);
    assert_eq!(
        sources
            .enter_file(files.path().join("./source"), None)
            .unwrap()
            .source,
        uuid
    );
    assert!(matches!(
        sources.enter_file(&path, Some(&root)),
        Err(CodeSourceError::ImportCycle { .. })
    ));
    assert_eq!(sources.sources.len(), 1);
    assert!(matches!(
        sources.get_source(&uuid),
        Some(CodeSource::File(_))
    ));
}

#[test]
fn file_import_keeps_inline_parent_available() {
    let files = Files::new();
    let uuid = Uuid::new_v4();
    let mut sources = CodeSources::unbound("inline", &uuid);
    let root = CodeSourceContext::new(uuid);
    let child = sources
        .enter_file(files.write("source", ""), Some(&root))
        .unwrap();
    assert_ne!(child.source, uuid);
    assert_eq!(child.ancestry, vec![uuid, child.source]);
    assert_eq!(sources.get_content(&uuid).unwrap().unwrap(), "inline");
    assert_eq!(sources.sources.len(), 2);
}

#[cfg(unix)]
#[test]
fn symlink_aliases_preserve_identity_and_cannot_hide_cycles() {
    let files = Files::new();
    let original = files.write("original", "");
    let alias = files.path().join("alias");
    std::os::unix::fs::symlink(&original, &alias).unwrap();
    let mut sources = CodeSources::default();
    let root = sources.enter_file(&original, None).unwrap();
    assert_eq!(
        sources.enter_file(&alias, None).unwrap().source,
        root.source
    );
    assert!(matches!(
        sources.enter_file(&alias, Some(&root)),
        Err(CodeSourceError::ImportCycle { .. })
    ));
}

#[test]
fn failed_registration_does_not_leave_a_source() {
    let files = Files::new();
    let mut sources = CodeSources::default();
    assert!(sources
        .enter_file(files.path().join("missing"), None)
        .is_err());
    assert!(sources.sources.is_empty());
    assert!(sources.files.is_empty());
}
