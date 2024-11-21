use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(function_declaration, FunctionDeclaration, 10);
