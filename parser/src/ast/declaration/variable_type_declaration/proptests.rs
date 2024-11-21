use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(variable_type_declaration, VariableTypeDeclaration, 10);
