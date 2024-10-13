use crate::{
    elements::{Command, Element, ElementId, InnersGetter},
    test_block, test_reading_ln_by_ln,
};

impl InnersGetter for Command {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

test_reading_ln_by_ln!(
    reading,
    &include_str!("../../../tests/reading/command.sibs"),
    &[ElementId::Command],
    130
);

test_block!(
    success,
    r#"
            $status = `./target/debug/exit 0 100 200 10`;
            $status.is_success();
        "#,
    true
);

test_block!(
    fail,
    r#"
            $status = `./target/debug/exit 1 100 200 10`;
            $status.is_fail();
        "#,
    true
);
