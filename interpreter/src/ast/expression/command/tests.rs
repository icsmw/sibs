use crate::*;

test_value_expectation!(
    command_000,
    Block,
    RtValue::Bool(true),
    r#"
    {
        `ls -lsa`.is_success();
    }"#
);

test_value_expectation!(
    command_001,
    Block,
    RtValue::Bool(true),
    r#"
    {
        `ls -lsa`.status::is_success();
    }"#
);

test_value_expectation!(
    command_002,
    Block,
    RtValue::Bool(true),
    r#"
    {
        `same very fake command`.status::is_failed();
    }"#
);

test_value_expectation!(
    command_003,
    Block,
    RtValue::Bool(false),
    r#"
    {
        `ls -lsa`.success().is_failed();
    }"#
);

test_value_expectation!(
    command_004,
    Block,
    RtValue::Bool(true),
    r#"
    {
        // Command was runned, but finished with error
        `ls -fake`.executed().is_failed();
    }"#
);

test_fail!(
    command_005,
    Block,
    r#"{
        // Failed because command is invalid
        `same very fake command`.success();
    }"#
);

test_fail!(
    command_006,
    Block,
    r#"{
        // Failed because command is invalid
        `same very fake command`.executed();
    }"#
);


test_fail!(
    command_007,
    Block,
    r#"{
        // Failed because arguments are invalid
        `ls -fake`.success();
    }"#
);

