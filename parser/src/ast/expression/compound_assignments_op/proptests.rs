use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(compound_assignments_op, CompoundAssignmentsOp, 10);
