use crate::*;

test_success!(
    variable_declaration_num_000,
    Block,
    r#"{ let a = 4; a = 5; }"#
);

test_success!(
    variable_declaration_num_001,
    Block,
    r#"{ let a: num = 4; a = 5; }"#
);

test_success!(
    variable_declaration_num_002,
    Block,
    r#"{ let a: num; a = 5; }"#
);

test_success!(
    variable_declaration_str_000,
    Block,
    r#"{ let a = "one"; a = "two"; }"#
);

test_success!(
    variable_declaration_str_001,
    Block,
    r#"{ let a: str = "one"; a = "two"; }"#
);

test_success!(
    variable_declaration_success_str_002,
    Block,
    r#"{ let a: str; a = "two"; }"#
);

test_success!(
    variable_declaration_success_redeclare_000,
    Block,
    r#"{ let a = "one"; let a = 1; }"#
);

test_success!(
    variable_declaration_success_redeclare_001,
    Block,
    r#"{ let a: str = "one"; let a: num = 1; }"#
);

test_success!(
    variable_declaration_success_redeclare_002,
    Block,
    r#"{ let a: str; let a: num; a = 1; }"#
);

test_success!(
    variable_declaration_success_scope_000,
    Block,
    r#"{ let a: str = "one"; if a == "one" { let a = 1; a = 2; }; a = "two"; }"#
);

test_fail!(
    variable_declaration_fail_mixed_000,
    Block,
    r#"{ let a = 4; a = "f"; }"#
);

test_fail!(
    variable_declaration_fail_mixed_001,
    Block,
    r#"{ let a = "4"; a = 4; }"#
);

test_fail!(
    variable_declaration_fail_mixed_002,
    Block,
    r#"{ let a: str; a = 5; }"#
);

test_fail!(
    variable_declaration_fail_mixed_003,
    Block,
    r#"{ let a: num; a = "f"; }"#
);

test_fail!(
    variable_declaration_fail_mixed_004,
    Block,
    r#"{ let a = true; a = "f"; }"#
);

test_fail!(
    variable_declaration_fail_mixed_005,
    Block,
    r#"{ let a: bool; a = "f"; }"#
);

test_fail!(
    variable_declaration_success_redeclare_000,
    Block,
    r#"{ let a = "one"; let a = 1; a = "two"; }"#
);

test_fail!(
    variable_declaration_success_redeclare_001,
    Block,
    r#"{ let a: str = "one"; let a: num = 1; a = "two"; }"#
);

test_fail!(
    variable_declaration_success_redeclare_002,
    Block,
    r#"{ let a: str; let a: num; a = 1; a = "two"; }"#
);

test_fail!(
    variable_declaration_success_scope_000,
    Block,
    r#"{ let a: str = "one"; if a == "one" { let a = 1; a = 2; }; a = 3; }"#
);
