use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(gatekeeper, Gatekeeper, 10);

// test_node_reading_case!(gatekeeper_case, Gatekeeper, "#[skip([*,2], func())]");
