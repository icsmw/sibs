use crate::{
    elements::{Element, ElementRef, InnersGetter, Optional},
    test_process_tasks_one_by_one, test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for Optional {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.condition.as_ref(), self.action.as_ref()]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../tests/reading/optional.sibs"),
    &[ElementRef::Optional],
    106
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../tests/error/optional.sibs"),
    &[ElementRef::Optional],
    7
);

test_process_tasks_one_by_one!(
    processing,
    &include_str!("../../tests/processing/optional.sibs"),
    true,
    3
);
