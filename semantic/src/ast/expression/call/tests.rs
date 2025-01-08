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
            let aaa = 5;
            aaa.test::sum(10);
        }
    };
    "#
);

test_success!(
    call_001,
    Anchor,
    r#"
    mod test {
        fn sum(a: num, b: num) {
            let c = a + b;
            c;
        };
        fn main() {
            a.sum(10);
            a.test::sum(10);
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

test_success!(
    call_002,
    Anchor,
    r#"
    mod aaa {
        fn sum(a: num, b: num) {
           a + b;
        };
        mod bbb {
            fn diff(a: num, b: num) {
                a - b;
            };
            fn main() {
                let a = 1;
                a.aaa::sum(10);
                a.diff(10)
                a.aaa::bbb::diff(10)
            };
        };
    };
    component my_component() {
        task task_a() {
            let a = 5;
            a.aaa::sum(10);
            a.aaa::bbb::diff(10);
        }
    };
    "#
);

test_success!(
    embedded_fn_call_000,
    Block,
    r#"{ let a = "Hello"; a.console::print(); }"#
);

test_success!(embedded_fn_call_001, Block, "{ let a = 5; a.sum(5); }");
test_success!(
    embedded_fn_call_002,
    Block,
    r#"{ let a = "R"; a.repeat(5); }"#
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
