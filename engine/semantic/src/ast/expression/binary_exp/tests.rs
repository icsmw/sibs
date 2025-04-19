use crate::*;

test_success!(
    binary_exp_000,
    Block,
    r#"{ let a = 1 / 2; let a = 1 + 2; let a = 1 - 2; let a = 1 * 2;}"#
);

test_success!(
    binary_exp_001,
    Block,
    r#"{ let a = 1; let b = 1; let c = a - b; let c = a + b; let c = a / b; let c = a * b;}"#
);

test_fail!(
    binary_exp_000,
    Block,
    r#"{ let a = 1; let b = "2"; let c = a * b; }"#
);

test_fail!(
    binary_exp_001,
    Block,
    r#"{ let a = 1; let b = true; let c = a * b; }"#
);

test_fail!(
    binary_exp_002,
    Block,
    r#"{ let a = 1; let b = [1,2]; let c = a * b; }"#
);
