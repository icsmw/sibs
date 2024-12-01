use crate::*;

test_success!(
    accessor_000,
    Block,
    r#"{ let a = [1,2,3,4]; let b = a[1]; }"#
);

test_success!(
    accessor_002,
    Block,
    r#"{ let a = [1,2,3,4]; let b = a["1"]; }"#
);

test_fail!(
    accessor_000,
    Block,
    r#"{ let a = [1,2,3,4]; let b = a["1"]; }"#
);
