use super::*;
use std::{borrow::Cow, collections::HashSet};

#[test]
fn descriptions_belong_to_their_list_items() {
    let docs = "# Arguments\n* `empty` -\n\nUnrelated paragraph.\n\n* `multiline`:\n\n  An indented continuation.\n* `lazy` -\nLazy continuation without a blank line.\n* `inline` — **Description** with `code`.\n";
    assert_eq!(
        documented_arguments(docs),
        HashSet::from([
            Cow::Borrowed("multiline"),
            Cow::Borrowed("lazy"),
            Cow::Borrowed("inline"),
        ])
    );
}

#[test]
fn code_blocks_and_nested_lists_do_not_declare_arguments() {
    let docs = "# Arguments\n\n    * `indented` - Example only.\n\n```sibs\n* `fenced` - Example only.\n```\n\n* `actual` - Description.\n  * `nested` - Detail, not another argument.\n\n> * `quoted` - Example only.\n";
    assert_eq!(
        documented_arguments(docs),
        HashSet::from([Cow::Borrowed("actual")])
    );
    assert!(
        documented_arguments("# Arguments\n*\n  * `nested_only` - Not a parameter.\n").is_empty()
    );
}

#[test]
fn arguments_require_names_descriptions_and_the_right_section() {
    let docs = "# Examples\n* `example` - Example only.\n\n## Arguments\n* `first` — First argument.\n- `second`:\n  Second argument.\n+ `empty` -\n* `also_empty`\n* Prose mentions `prose`, but does not name an argument.\n\n# Examples\n* `outside` - Example only.\n";
    assert_eq!(
        documented_arguments(docs),
        HashSet::from([Cow::Borrowed("first"), Cow::Borrowed("second"),])
    );
}

#[test]
fn ordinary_names_are_borrowed_and_unicode_descriptions_are_supported() {
    let names = documented_arguments("Arguments\n=========\n* `message` — Сообщение.\n");
    assert!(matches!(
        names.get("message"),
        Some(Cow::Borrowed("message"))
    ));
}
