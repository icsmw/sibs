use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(comparison_seq, ComparisonSeq, 10);
