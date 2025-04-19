use crate::*;

test_success!(
    binary_exp_group_000,
    Block,
    r#"{ let a = 4; let b = 5; let c = (a * b / a - a + a) * 2; }"#
);

test_success!(
    binary_exp_group_001,
    Block,
    r#"{ let a = 4; let b = 5; let c = (a * b) / (a - a + a); }"#
);

test_fail!(
    binary_exp_group_000,
    Block,
    r#"{ let a = 4; let b = "5"; let c = (a * b / a) - (a * 2); }"#
);

test_fail!(
    binary_exp_group_001,
    Block,
    r#"{ let a = true; let b = 5; let c = (a * b / a - a + a); }"#
);

test_fail!(
    binary_exp_group_002,
    Block,
    r#"{ let a = [1,2,3]; let b = "5"; let c = 1 * a * (b / a - a + a); }"#
);
