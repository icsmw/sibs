use crate::*;

test_task_results_from_file!(
    globals_booleans,
    "globals_test",
    "booleans",
    RtValue::Bool(false),
    "../tests/globals/main.sibs"
);

test_task_results_from_file!(
    globals_arithmetic,
    "globals_test",
    "arithmetic",
    RtValue::Num(9.0),
    "../tests/globals/main.sibs"
);
test_task_results_from_file!(
    globals_functions,
    "globals_test",
    "functions",
    RtValue::Num(6.0),
    "../tests/globals/main.sibs"
);
test_task_results_from_file!(
    globals_strings,
    "globals_test",
    "strings",
    RtValue::Str("start_done".into()),
    "../tests/globals/main.sibs"
);
test_task_results_from_file!(
    globals_arrays,
    "globals_test",
    "arrays",
    RtValue::Vec(vec![RtValue::Num(2.0), RtValue::Num(3.0)]),
    "../tests/globals/main.sibs"
);
test_task_results_from_file!(
    globals_interpolation,
    "globals_test",
    "interpolation",
    RtValue::Str("base 2".into()),
    "../tests/globals/main.sibs"
);
test_task_results_from_file!(
    globals_assignment,
    "globals_test",
    "assignment",
    RtValue::Num(42.0),
    "../tests/globals/main.sibs"
);
test_task_results_from_file!(
    globals_sharing,
    "globals_test",
    "sharing",
    RtValue::Num(12.0),
    "../tests/globals/main.sibs"
);
test_task_results_from_file!(
    globals_parallel,
    "globals_test",
    "parallel",
    RtValue::Num(102.0),
    "../tests/globals/main.sibs"
);
test_task_results_from_file!(
    globals_rhs_once,
    "globals_test",
    "rhs_once",
    RtValue::Num(6.0),
    "../tests/globals/main.sibs"
);

test_task_results_from_file!(
    env_text,
    "env_test",
    "text",
    RtValue::Str("123 Юникод with spaces".into()),
    "../tests/globals/env_main.sibs",
    env = [
        (
            "SIBS_TEST_GLOBAL_TEXT",
            Some("123 Юникод with spaces".into())
        ),
        ("SIBS_TEST_GLOBAL_EMPTY", Some("".into()))
    ]
);
test_task_results_from_file!(
    env_empty,
    "env_test",
    "empty",
    RtValue::Str("".into()),
    "../tests/globals/env_main.sibs",
    env = [
        ("SIBS_TEST_GLOBAL_TEXT", Some("different run".into())),
        ("SIBS_TEST_GLOBAL_EMPTY", Some("".into()))
    ]
);
test_task_results_from_file!(
    env_interpolation,
    "env_test",
    "interpolation",
    RtValue::Str("prefix 007".into()),
    "../tests/globals/env_main.sibs",
    env = [
        ("SIBS_TEST_GLOBAL_TEXT", Some("007".into())),
        ("SIBS_TEST_GLOBAL_EMPTY", Some("".into()))
    ]
);
test_task_error_from_file!(
    env_missing_stops_startup,
    "env_test",
    "must_not_run",
    E::EnvironmentNotDefined(_),
    "../tests/globals/env_main.sibs",
    env = [
        ("SIBS_TEST_GLOBAL_TEXT", None),
        ("SIBS_TEST_GLOBAL_EMPTY", Some("".into()))
    ]
);

#[cfg(unix)]
test_task_error_from_file!(
    env_non_unicode,
    "env_test",
    "must_not_run",
    E::EnvironmentNotUnicode(_),
    "../tests/globals/env_main.sibs",
    env = [
        (
            "SIBS_TEST_GLOBAL_TEXT",
            Some(<std::ffi::OsString as std::os::unix::ffi::OsStringExt>::from_vec(vec![0xff]))
        ),
        ("SIBS_TEST_GLOBAL_EMPTY", Some("".into()))
    ]
);
