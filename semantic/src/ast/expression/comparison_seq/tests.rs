use crate::*;

test_success!(
    comparison_seq_000,
    Block,
    r#"{ let a = 4; let b = 5; let c = a == b }"#
);

test_success!(
    comparison_seq_001,
    Block,
    r#"{ let a = 4; let b = 5; let c = if a == b { a; } else { b; }; }"#
);

test_success!(
    comparison_seq_002,
    Block,
    r#"{ let a = 4; let b = 5; let c = true; let d = if c { a; } else { b; }; }"#
);

test_fail!(
    comparison_seq_000,
    Block,
    r#"{ let a = 4; let b = 5; let c = if a { a; } else { b; }; }"#
);

test_fail!(
    comparison_seq_001,
    Block,
    r#"{ let a = 4; let b = 5; let c = if a > b { "a"; } else { 5; }; }"#
);
