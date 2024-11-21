use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(variable_variants, VariableVariants, 10);
