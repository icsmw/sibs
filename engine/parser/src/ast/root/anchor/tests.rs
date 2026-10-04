use crate::ast::tests::Files;
use crate::*;

test_node_grammar!(
    declarations_are_not_working_file_statements,
    Anchor::read,
    accepts = [],
    rejects = [
        "global count: num = 0;",
        "const count: num = 0;",
        "mod m { global count: num = 0; }",
        "component c() { task t() { const count: num = 0; } }",
        "globals \"x\"",
        "globals from",
        "envs \"x\"",
        "envs from",
    ],
);

#[test]
fn incompatible_body_kinds_are_rejected_even_for_empty_files() {
    let files = Files::new();
    files.write("body.sibs", "");
    let main = files.write(
        "main.sibs",
        "globals from \"body.sibs\"; envs from \"body.sibs\";",
    );
    assert!(Anchor::read(&Parser::new(main, false).unwrap()).is_err());
}
