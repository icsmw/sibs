use crate::{
    elements::{Element, ElementRef, InnersGetter, VariableName},
    test_reading_el_by_el, test_reading_with_errors_line_by_line,
};

impl InnersGetter for VariableName {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/variable_name.sibs"),
    ElementRef::VariableName,
    3
);

test_reading_with_errors_line_by_line!(
    errors,
    &include_str!("../../../tests/error/variable_name.sibs"),
    ElementRef::VariableName,
    3
);
