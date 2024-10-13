use crate::{
    elements::{Comparing, Element, ElementRef, InnersGetter},
    test_reading_ln_by_ln, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for Comparing {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.left.as_ref(), self.right.as_ref()]
    }
}

test_reading_ln_by_ln!(
    reading,
    &include_str!("../../../tests/reading/comparing.sibs"),
    &[ElementRef::Comparing],
    190
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../../tests/error/comparing.sibs"),
    &[ElementRef::Comparing],
    11
);
