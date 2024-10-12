use crate::{
    elements::{Element, ElementRef, InnersGetter, Reference},
    test_reading_el_by_el, test_reading_with_errors_line_by_line,
};

impl InnersGetter for Reference {
    fn get_inners(&self) -> Vec<&Element> {
        self.inputs.iter().collect()
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../tests/reading/refs.sibs"),
    ElementRef::Reference,
    6
);

test_reading_with_errors_line_by_line!(
    errors,
    &include_str!("../../tests/error/refs.sibs"),
    ElementRef::Reference,
    8
);
