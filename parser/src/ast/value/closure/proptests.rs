use crate::*;
use proptest::prelude::*;

test_node_reading!(Closure, 10);

// test_node_reading_case!(
//     closure_case,
//     Block,
//     r#"
// {
//     some::func(5,4, |a: num | { a + a;});
// }
// "#
// );
