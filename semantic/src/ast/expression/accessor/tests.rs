use crate::*;

test_success!(
    accessor_000,
    Block,
    r#"{ let a = [1,2,3,4]; let b = a[1]; }"#
);

test_success!(
    accessor_001,
    Block,
    r#"{ let a = [1,2,3,4]; let n = 1; let b = a[n]; }"#
);

test_success!(
    accessor_002,
    Block,
    r#"{ let a = ["1","2","3","4"]; let n = 1; let b: str = a[n]; }"#
);

test_fail!(
    accessor_000,
    Block,
    r#"{ let a = [1,2,3,4]; let n = "1"; let b = a[n]; }"#
);

test_fail!(
    accessor_001,
    Block,
    r#"{ let a = ["1","2","3","4"]; let n = 1; let b: num = a[n]; }"#
);