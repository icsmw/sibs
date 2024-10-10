use crate::{
    elements::{function::Function, Element, ElementRef, InnersGetter},
    test_process_tasks_one_by_one, test_reading_line_by_line,
    test_reading_with_errors_line_by_line,
};

impl InnersGetter for Function {
    fn get_inners(&self) -> Vec<&Element> {
        self.args.iter().collect()
    }
}

test_reading_line_by_line!(
    reading,
    &include_str!("../../tests/reading/function.sibs"),
    ElementRef::Function,
    107
);

test_reading_with_errors_line_by_line!(
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
