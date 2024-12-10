use crate::*;
use proptest::prelude::*;

test_node_reading!(Variable, 10);

// test_node_reading_case!(
//     block_case,
//     Block,
//     r#"{
//         ! g9ja7 && false > false => return false ;

//         if !a && b { !b; };
//         let a = !b;
//         `asdasdas{!b}dasds`;
//     }"#
// );
