use crate::*;
use proptest::prelude::*;

test_node_reading!(Include, 10);

// test_node_reading_case!(
//     block_case,
//     Include,
//     r#"
// include
// /// fdsfsdfsd
// // fsdfsdfsd
// "somepath"
// "#
// );
