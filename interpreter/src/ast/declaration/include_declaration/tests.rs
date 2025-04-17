use crate::*;

test_task_results_from_file!(
    include_declaration_000,
    "component_a",
    "task_a",
    RtValue::Bool(true),
    "../tests/mods/main.sibs"
);
