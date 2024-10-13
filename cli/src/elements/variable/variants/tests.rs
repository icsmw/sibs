use crate::{
    elements::{Element, ElementId, InnersGetter, VariableVariants},
    test_reading_ln_by_ln, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for VariableVariants {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}

test_reading_ln_by_ln!(
    reading,
    &include_str!("../../../tests/reading/variants.sibs"),
    &[ElementId::VariableVariants],
    3
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../../tests/error/variants.sibs"),
    &[ElementId::VariableVariants],
    4
);
