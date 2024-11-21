use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(r#return, Return, 10);

// test_node_reading_case!(return_case, Return, "return ");
