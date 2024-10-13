use crate::{
    elements::{Element, ElementId, IfSubsequence, InnersGetter},
    test_reading_ln_by_ln,
};

impl InnersGetter for IfSubsequence {
    fn get_inners(&self) -> Vec<&Element> {
        self.subsequence.iter().collect()
    }
}

test_reading_ln_by_ln!(
    reading,
    &include_str!("../../../../tests/reading/subsequence.sibs"),
    &[ElementId::IfSubsequence],
    86
);
