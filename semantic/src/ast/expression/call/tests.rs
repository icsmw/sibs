use crate::*;

test_success!(
    call_000,
    Anchor,
    r#"
    mod test {
        fn sum(a: num, b: num) {
            let c = a + b;
            c;
        };
    };
    component my_component() {
        task task_a() {
            let a = 5;
            a.test::sum(10);
        }
    };
    "#
);

test_fail!(
    call_000,
    Anchor,
    r#"
    mod test {
        fn sum(a: num, b: num) {
            let c = a + b;
            c;
        };
    };
    component my_component() {
        task task_a() {
            let a = "5";
            a.test::sum(10);
        }
    };
    "#
);
