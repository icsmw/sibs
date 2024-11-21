use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(variable_declaration, VariableDeclaration, 10);
