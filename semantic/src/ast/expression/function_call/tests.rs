use crate::*;

test_success!(
    function_call_000,
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

test_success!(
    function_call_001,
    Anchor,
    r#"
    mod aaa {
        fn get_num(a: num, b: bool) {
            a;
        };
        fn get_bool(a: num, b: bool) {
            let r = get_num(a, b);
            let t = aaa::get_num(a, b);
            b;
        };
    };
    component my_component() { 
        task task_a() {
            // Nothing
            let a = 1;
        }
    };
    "#
);

test_success!(
    function_call_002,
    Anchor,
    r#"
    mod aaa {
        fn get_num(a: num, b: bool) {
            let t = aaa::bbb::get_bool(a, b);
            a;
        };
        mod bbb {
            fn get_bool(a: num, b: bool) {
                let r = aaa::get_num(a, b);
                let t = aaa::bbb::get_bool(a, b);
                b;
            };
            fn recurstion(a: num, b: bool) {
                aaa::bbb::recurstion(a, b);
            };
            fn recurstion_if(a: num, b: bool) {
                if a == 2 {
                    aaa::bbb::recurstion_if(a, b);
                } else {
                    5
                }
            };
        };
    };
    component my_component() { 
        task task_a() {
            let t = aaa::bbb::get_bool(a, b);
        }
    };
    "#
);
test_fail!(
    function_call_000,
    Anchor,
    r#"
    mod test {
        fn get_num(a: num, b: bool) {
            a;
        };
    };
    component my_component() { 
        task task_a() {
            let r: str = test::get_num(1, true);
        }
    };
    "#
);

test_finalize_fail!(
    function_call_000,
    Anchor,
    r#"
    mod test {
        fn get_num(a: num, b: bool) {
            a;
        };
    };
    component my_component() { 
        task task_a() {
            test::get_num("1", true);
        }
    };
    "#
);

test_finalize_fail!(
    function_call_001,
    Anchor,
    r#"
    mod test {
        fn get_num(a: num, b: bool) {
            a;
        };
    };
    component my_component() { 
        task task_a() {
            test::get_num(1, "true");
        }
    };
    "#
);
