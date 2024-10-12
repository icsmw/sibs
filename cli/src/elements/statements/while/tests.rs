use crate::{
    elements::{Element, ElementRef, InnersGetter, While},
    test_block, test_reading_el_by_el,
};

impl InnersGetter for While {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.block.as_ref(), self.condition.as_ref()]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/while.sibs"),
    ElementRef::While,
    1
);

test_block!(
    simple,
    r#"
        $n = 0;
        while $n < 10 {
            $n += 1;
        };
        $n;
    "#,
    10isize
);

test_block!(
    with_break,
    r#"
        $n = 0;
        while $n < 10 {
            $n += 1;
            if $n == 5 {
                break;
            };
        };
        $n;
    "#,
    5isize
);
