use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(InterpolatedString, 10);

// test_node_reading_case!(
//     interpolated_string_case,
//     InterpolatedString,
//     "'test of string { if a > 4 {  }  } string'"
// );
