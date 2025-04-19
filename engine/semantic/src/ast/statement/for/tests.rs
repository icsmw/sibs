use crate::*;

test_success!(
    for_000,
    Block,
    r#"{
        let sum: num = 1.4;
        for (el, n) in 0..10 {
            sum += el;
        };
    }"#
);

test_success!(
    for_001,
    Block,
    r#"{
        let list: Vec<str> = ["one", "two", "three"];
        let concat = "";
        let sum: num = 0;
        for (el, n) in list {
            concat = '{concat}{el}';
            sum += n;
        };
    }"#
);

test_success!(
    for_002,
    Block,
    r#"{
        let fullstr = "one two three";
        let result = "";
        for (ch, n) in fullstr {
            result = '{result}{ch}';
        };
    }"#
);

test_fail!(
    for_000,
    Block,
    r#"{
        let wrong = 10;
        let sum: num = 0;
        for (el, n) in wrong {
            sum += n;
        };
    }"#
);
