use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(Block, 10);

// test_node_reading_case!(
//     block_case,
//     Block,
//     r#"{
//     // comment
//     return ; break ; a = 'str { if a > 5 {
//         // commentA
//         v = 111 ;
//     } else {
//         // commentB
//         a = 222 ;
//     } } str' ; l += 1 ; // comment }"#
// );
