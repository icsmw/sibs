use crate::*;

test_value_expectation!(
    repeat_000,
    Block,
    RtValue::Str(String::from("RRRRR")),
    r#"{
        "R".strs::repeat(5);
    }"#
);

test_value_expectation!(
    repeat_001,
    Block,
    RtValue::Str(String::from("RRRRR")),
    r#"{
        let r = "R";
        r.repeat(5);
    }"#
);

test_value_expectation!(
    repeat_002,
    Block,
    RtValue::Bool(true),
    r#"{
        "R".repeat(5) == "RRRRR";
    }"#
);

// test_block!(
//     to_ascii_lowercase,
//     r#"
//             if "R".to_ascii_lowercase() == "r" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     to_ascii_uppercase,
//     r#"
//             if "r".to_ascii_uppercase() == "R" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     to_lowercase,
//     r#"
//             if "R".to_lowercase() == "r" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     to_uppercase,
//     r#"
//             if "r".to_uppercase() == "R" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     sub,
//     r#"
//             $a = "Hello World!";
//             $b = $a.sub(0, 5);
//             $c = $a.str::sub(0, 5).str::sub(0, 2);
//             if $b == "Hello" && $c == "He" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     split_off,
//     r#"
//             if "Hello, World!".split_off(7) == "World!" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     trim,
//     r#"
//             if "   word   ".trim() == "word" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     trim_end,
//     r#"
//             if "   word   ".trim_end() == "   word" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     trim_start,
//     r#"
//             if "   word   ".trim_start() == "word   " {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//     true
// );

// test_block!(
//     len,
//     r#"
//             "12345".len() == 5;
//         "#,
//     true
// );

// test_block!(
//     is_empty,
//     r#"
//             "".is_empty();
//         "#,
//     true
// );

// test_block!(
//     is_trimmed_empty,
//     r#"
//             "   ".is_trimmed_empty();
//         "#,
//     true
// );
