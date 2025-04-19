use crate::*;
use proptest::prelude::*;

test_node_reading!(Block, 10);

// test_node_reading_case!(
//     block_case,
//     Block,
//     r#"{
//         let a = 4;
//         Error(
//             /// Hello
//             // comment
//             "aaa"
//         );
//     }"#
// );
