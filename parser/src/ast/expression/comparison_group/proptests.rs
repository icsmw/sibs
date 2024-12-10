use crate::*;
use proptest::prelude::*;

test_node_reading!(ComparisonGroup, 10);

// test_node_reading_case!(
//     block_case,
//     Block,
//     r#"{
//         if !(a != b && c == 5) { !b; };
//     }"#
// );
