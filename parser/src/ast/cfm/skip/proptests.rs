use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(Skip, 10);

// test_node_reading_case!(skip_case, Skip, "skip([*,2], func())");
