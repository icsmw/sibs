use crate::*;

test_success!(
    while_000,
    Block,
    r#"{
        let n = 0;
        while (n < 10) {
            n += 1;
        };
    }"#
);

test_success!(
    while_001,
    Block,
    r#"{
        let n = 0;
        let i = 0;
        while n < 10 && i < 10 {
            n += 1;
            i += 2;
        };
    }"#
);

test_success!(
    while_002,
    Block,
    r#"{
        let a = true;
        while a {
            a = false;
        };
    }"#
);

test_success!(
    while_003,
    Block,
    r#"{
        while true {
            let a = false;
        };
    }"#
);
