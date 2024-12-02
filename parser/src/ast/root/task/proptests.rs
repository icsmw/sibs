use crate::*;
use asttree::*;
use proptest::prelude::*;

test_node_reading!(Task, 10);

// test_node_reading_case!(task_case, Task, "task name() { f = 4; }");
