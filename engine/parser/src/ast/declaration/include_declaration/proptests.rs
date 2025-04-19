use crate::*;
use proptest::prelude::*;

test_node_reading!(IncludeDeclaration, 10);

// test_node_reading_case!(
//     block_case,
//     IncludeDeclaration,
//     r#"
// include
// /// fdsfsdfsd
// // fsdfsdfsd
// "somepath"
// "#
// );
