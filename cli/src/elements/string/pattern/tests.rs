use crate::{
    elements::{Element, ElementId, InnersGetter, PatternString},
    test_reading_ln_by_ln,
};

impl InnersGetter for PatternString {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

test_reading_ln_by_ln!(
    reading,
    &include_str!("../../../tests/reading/pattern_string.sibs"),
    &[ElementId::PatternString],
    96
);
