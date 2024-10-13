use crate::{
    elements::{function::Function, Element, ElementRef, InnersGetter},
    test_process_tasks_one_by_one, test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for Function {
    fn get_inners(&self) -> Vec<&Element> {
        self.args.iter().collect()
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../tests/reading/function.sibs"),
    ElementRef::Function,
    107
);

test_reading_with_errors_ln_by_ln!(
    reading_with_errors,
    &include_str!("../../tests/error/function.sibs"),
    ElementRef::Function,
    7
);

test_process_tasks_one_by_one!(
    processing_test,
    &include_str!("../../tests/processing/functions.sibs"),
    true,
    4
);
