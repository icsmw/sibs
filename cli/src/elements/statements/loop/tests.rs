use crate::{
    elements::{Element, ElementId, InnersGetter, Loop},
    test_block, test_reading_el_by_el,
};

impl InnersGetter for Loop {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.block.as_ref()]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/loop.sibs"),
    &[ElementId::Loop],
    1
);

test_block!(
    normal_loop,
    r#"
        $n = 0;
        loop {
            $n += 1;
            if $n == 10 {
                break;
            };
        };
        $n;
    "#,
    10isize
);
