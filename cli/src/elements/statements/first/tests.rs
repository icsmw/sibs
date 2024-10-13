use crate::{
    elements::{Element, ElementId, First, InnersGetter},
    test_process_tasks_one_by_one, test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for First {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.block.as_ref()]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/first.sibs"),
    &[ElementId::First],
    2
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../../tests/error/first.sibs"),
    &[ElementId::First],
    3
);

test_process_tasks_one_by_one!(
    processing,
    &include_str!("../../../tests/processing/first.sibs"),
    true,
    4
);
