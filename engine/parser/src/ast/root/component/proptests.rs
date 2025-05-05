use crate::*;
use proptest::prelude::*;

test_node_reading!(Component, 10);

// test_node_reading_case!(
//     component_case,
//     Component,
//     r#"component component_a( jkjkkkkjkjk ) {
//     /// This description is task_a
//     task task_a(a: str) {
//         let a = 5;
//         a.fns::sum(a);
//     }
// };"#
// );
