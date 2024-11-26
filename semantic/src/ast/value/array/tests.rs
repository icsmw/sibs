use crate::*;

test_success!(
    array_000,
    Block,
    r#"{ let a: Vec<num> = [1,2]; a = [3,4]; }"#
);

test_success!(array_001, Block, r#"{ let a = [1,2]; a = [3,4]; }"#);

test_success!(array_002, Block, r#"{ let a = []; a = [1,2]; }"#);

test_success!(array_003, Block, r#"{ let a; a = [1,2]; }"#);

test_success!(array_004, Block, r#"{ let a: Vec<num>; a = [1,2]; }"#);

test_fail!(
    array_000,
    Block,
    r#"{ let a: Vec<num> = [1,2]; a = [true, false]; }"#
);

test_fail!(
    array_001,
    Block,
    r#"{ let a: Vec<num>; a = [true, false]; }"#
);

test_fail!(array_002, Block, r#"{ let a = [1,2]; a = [true, false]; }"#);

test_fail!(
    array_003,
    Block,
    r#"{ let a: Vec<num> = []; a = [true, false]; }"#
);
