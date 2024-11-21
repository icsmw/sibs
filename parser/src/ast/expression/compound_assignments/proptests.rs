use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(compound_assignments, CompoundAssignments, 10);
