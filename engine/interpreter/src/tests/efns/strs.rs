use crate::*;

test_value_expectation!(
    strs_repeat_000,
    Block,
    RtValue::Str(String::from("RRRRR")),
    r#"{
        "R".strs::repeat(5);
    }"#
);

test_value_expectation!(
    strs_repeat_001,
    Block,
    RtValue::Str(String::from("RRRRR")),
    r#"{
        let r = "R";
        r.repeat(5);
    }"#
);

test_value_expectation!(
    strs_repeat_002,
    Block,
    RtValue::Bool(true),
    r#"{
        "R".repeat(5) == "RRRRR";
    }"#
);

test_value_expectation!(
    strs_to_ascii_lowercase,
    Block,
    RtValue::Str(String::from("r")),
    r#"{
        "R".to_ascii_lowercase();
    }"#
);

test_value_expectation!(
    strs_to_ascii_uppercase,
    Block,
    RtValue::Str(String::from("R")),
    r#"{
        "r".to_ascii_uppercase();
    }"#
);

test_value_expectation!(
    strs_to_lowercase,
    Block,
    RtValue::Str(String::from("r")),
    r#"{
        "R".to_lowercase();
    }"#
);

test_value_expectation!(
    strs_to_uppercase,
    Block,
    RtValue::Str(String::from("R")),
    r#"{
        "r".to_uppercase();
    }"#
);

test_value_expectation!(
    strs_sub,
    Block,
    RtValue::Str(String::from("He")),
    r#"{
        let a = "Hello World!";
        let b = a.sub(0, 5);
        a.sub(0, 5).sub(0, 2);
    }"#
);

test_value_expectation!(
    strs_split_off,
    Block,
    RtValue::Str(String::from("World!")),
    r#"{
        "Hello, World!".split_off(7);
    }"#
);

test_value_expectation!(
    strs_trim,
    Block,
    RtValue::Str(String::from("word")),
    r#"{
        "   word   ".trim();
    }"#
);

test_value_expectation!(
    strs_trim_end,
    Block,
    RtValue::Str(String::from("   word")),
    r#"{
        "   word   ".trim_end();
    }"#
);

test_value_expectation!(
    strs_trim_start,
    Block,
    RtValue::Str(String::from("word   ")),
    r#"{
        "   word   ".trim_start();
    }"#
);

test_value_expectation!(
    strs_len,
    Block,
    RtValue::Num(5.0),
    r#"{
        "12345".len();
    }"#
);

test_value_expectation!(
    strs_is_empty,
    Block,
    RtValue::Bool(true),
    r#"{
        "".is_empty();
    }"#
);

test_value_expectation!(
    strs_is_trimmed_empty,
    Block,
    RtValue::Bool(true),
    r#"{
        "   ".is_trimmed_empty();
    }"#
);
