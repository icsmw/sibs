use crate::*;

test_success!(
    comparison_seq_000,
    Anchor,
    r#"
    mod test {
        fn get_num(a: num, b: bool) {
            a;
        };
        fn get_bool(a: num, b: bool) {
            b;
        }
    };
    component my_component() { 
        task task_a() {
            test::get_num(1, true);
        }
        task task_b() {
            test::get_bool(1, true);
        }
    };
    "#
);
