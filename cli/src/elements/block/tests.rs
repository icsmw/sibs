use crate::{
    elements::{Block, Element, ElementRef, InnersGetter},
    test_reading_el_by_el,
};

impl InnersGetter for Block {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

test_reading_el_by_el!(
    reading,
    &format!(
        "{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}",
        include_str!("../../tests/reading/if.sibs"),
        include_str!("../../tests/reading/variable_assignation.sibs"),
        include_str!("../../tests/reading/function.sibs"),
        include_str!("../../tests/reading/optional.sibs"),
        include_str!("../../tests/reading/each.sibs"),
        include_str!("../../tests/reading/refs.sibs")
    ),
    &[ElementRef::Block],
    6
);
