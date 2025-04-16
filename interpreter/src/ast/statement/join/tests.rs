use crate::*;

test_value_expectation!(
    join_000,
    Block,
    RtValue::Bool(true),
    r#"
    {
        join(
            `../target/debug/exit 0 500 60 60`,
            `../target/debug/exit 0 400 60 60`,
            `../target/debug/exit 0 100 60 60`,
            `../target/debug/exit 0 200 60 60`,
            `../target/debug/exit 0 300 60 60`,
        );
        true;
    }"#
);
