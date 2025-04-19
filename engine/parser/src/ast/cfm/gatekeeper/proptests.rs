use crate::*;
use proptest::prelude::*;

test_node_reading!(Gatekeeper, 10);

// test_node_reading_case!(gatekeeper_case, Gatekeeper, "#[skip([*,2], func())]");
