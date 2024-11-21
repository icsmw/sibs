use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(function_call, FunctionCall, 10);
