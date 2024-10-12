use crate::{
    elements::{Element, For, InnersGetter},
    test_block, test_reading_el_by_el,
};

impl InnersGetter for For {
    fn get_inners(&self) -> Vec<&Element> {
        vec![
            self.block.as_ref(),
            self.index.as_ref(),
            self.target.as_ref(),
        ]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/for.sibs"),
    crate::elements::ElementRef::For,
    5
);

test_block!(
    no_steps,
    r#"
        for $n in 0..0 {
            print($n);
        };
        $n;
    "#,
    0isize
);

test_block!(
    increase_index,
    r#"
        for $n in 0..10 {
            print($n);
        };
        true;
    "#,
    true
);

test_block!(
    reduce_index,
    r#"
        for $n in 10..0 {
            print($n);
        };
        true;
    "#,
    true
);

test_block!(
    increase_index_break,
    r#"
        for $n in 0..10 {
            if $n == 5 {
                break;
            };
            print($n);
        };
        if $n == 5 {
            true;
        } else {
            false;
        };
    "#,
    true
);

test_block!(
    reduce_index_and_incrementer,
    r#"
        $i = 10;
        for $n in 10..0 {
            print($n);
            $i -= 1;
        };
        if $i == 0 {
            true;
        } else {
            false;
        };
    "#,
    true
);

test_block!(
    increase_index_and_incrementer,
    r#"
        $i = 0;
        for $n in 0..10 {
            print($n);
            $i += 1;
        };
        if $i == 10 {
            true;
        } else {
            false;
        };
    "#,
    true
);

test_block!(
    reduce_index_break,
    r#"
        for $n in 10..0 {
            if $n == 5 {
                break;
            };
            print($n);
        };
        if $n == 5 {
            true;
        } else {
            false;
        };
    "#,
    true
);

test_block!(
    iteration,
    r#"
        for $el in ("one", "two", "three") {
            print($el);
        };
        true;
    "#,
    true
);

test_block!(
    iteration_from_var,
    r#"
        $els = ("one", "two", "three");
        for $el in $els {
            print($el);
        };
        true;
    "#,
    true
);
