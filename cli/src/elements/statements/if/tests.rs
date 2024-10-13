use crate::{
    elements::{Element, ElementRef, If, InnersGetter},
    test_process_tasks_one_by_one, test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for If {
    fn get_inners(&self) -> Vec<&Element> {
        self.threads
            .iter()
            .flat_map(|thr| thr.get_inners())
            .collect()
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/if.sibs"),
    &[ElementRef::If],
    90
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../../tests/error/if.sibs"),
    &[ElementRef::If],
    15
);

test_process_tasks_one_by_one!(
    processing,
    &include_str!("../../../tests/processing/if.sibs"),
    true,
    99
);
