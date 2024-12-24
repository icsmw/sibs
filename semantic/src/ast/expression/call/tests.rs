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
/**

    mod aaa {
       fn sum(a: num, b: num) {
           let c = a + b;
           c;
       };
       mod bbb {
           fn diff(a: num, b: num) {
               a - b;
           };
           fn main() {
               /// Should be fail, because no "a" in the scope!
               a.aaa::sum(10);
               a.diff(10)
               a.aaa::bbb::diff(10)
           };
       };
   };
*
*/
test_success!(
    call_002,
    Anchor,
    r#"
    mod aaa {
        fn sum(a111: num, b: num) {
           let c = a111 + b;
           c;
        };
        mod bbb {
            fn diff(a: num, b: num) {
                a - b;
            };
            fn main() {
                a111.aaa::sum(10);
                a111.diff(10)
                a111.aaa::bbb::diff(10)
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
