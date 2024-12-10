use crate::*;

test_success!(
    comparison_group_000,
    Block,
    r#"{ let a = 4; let b = 5; let c = "yes"; let d = (a == b) && (1 == 2) || (c == "yes") }"#
);

test_success!(
    comparison_group_001,
    Block,
    r#"{ let a = 4; let b = 5; let c = "yes"; let d = if (a == b) && (1 == 2) || (c == "yes") { true; } else { false; } }"#
);

test_success!(
    comparison_group_002,
    Block,
    r#"{ let a = 4; let b = 5; let c = "yes"; let d = if ((a == b) && (1 == 2) || (c == "yes") && a == 7) { true; } if ((a != b) && (1 >= 2) || (c != "yes") && a <= 7) { true } else { false; } }"#
);

test_success!(
    comparison_group_003,
    Block,
    r#"{ let a = true; let b = false; let c = if a && b { "yes"; } else { "no"; }; }"#
);

test_success!(
    comparison_group_004,
    Block,
    r#"{ let x = 42; let y = 42; let z = if x == y { x; } else { y; }; }"#
);

test_success!(
    comparison_group_005,
    Block,
    r#"{ let str1 = "hello"; let str2 = "hello"; let result = if str1 == str2 { true; } else { false; }; }"#
);

test_success!(
    comparison_group_006,
    Block,
    r#"{ let a = 10; let b = 20; let is_valid = if a < b && b > 0 { true; } else { false; }; }"#
);

test_success!(
    comparison_group_007,
    Block,
    r#"{ let a = 1; let b = 2; let c = 3; let result = if (a < b) && (b < c) { "ordered"; } else { "not ordered"; }; }"#
);

test_fail!(
    comparison_group_000,
    Block,
    r#"{ let a = 4; let b = 5; let c = if a == 5 || a { a; } else { b; }; }"#
);

test_fail!(
    comparison_group_001,
    Block,
    r#"{ let a = 4; let b = 5; let c = if ((a == 4) && (a > b)) || b == "5" { true; } else { false; }; }"#
);

test_fail!(
    comparison_group_002,
    Block,
    r#"{ let a = true; let b = "hello"; let result = if a && b { true; } else { false; }; }"#
);

test_fail!(
    comparison_group_003,
    Block,
    r#"{ let a = 10; let b = 20; let result = if a > b { 10; } else { "string"; }; }"#
);

test_fail!(
    comparison_group_004,
    Block,
    r#"{ let str_var = "test"; let num_var = 123; let result = num_var == str_var; }"#
);

test_fail!(
    comparison_group_005,
    Block,
    r#"{ let a = true; let result = if a { 42; } else { "not a number"; }; }"#
);

test_fail!(
    comparison_group_006,
    Block,
    r#"{ let a = 10; let b = "20"; let c = if a > b { true; } else { false; }; }"#
);
