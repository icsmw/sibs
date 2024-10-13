use crate::{
    elements::{Element, ElementId, For, Task},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for For {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((vec![ElementId::VariableName], deep)),
            Element::arbitrary_with((
                vec![ElementId::Range, ElementId::VariableName, ElementId::Values],
                deep,
            )),
            Element::arbitrary_with((vec![ElementId::Block], deep)),
        )
            .prop_map(|(index, target, block)| For {
                index: Box::new(index),
                target: Box::new(target),
                block: Box::new(block),
                token: 0,
            })
            .boxed()
    }
}

fn reading(instance: For) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{instance};\n}};");
        read_string!(
            &Configuration::logs(false),
            &origin,
            |reader: &mut Reader, src: &mut Sources| {
                let task = src
                    .report_err_if(Task::dissect(reader))?
                    .expect("Task read");
                assert_eq!(format!("{task};"), origin);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    })
}

proptest! {
    #![proptest_config(ProptestConfig {
        max_shrink_iters: 5000,
        ..ProptestConfig::with_cases(10)
    })]
    #[test]
    fn test_run_task(
        args in any_with::<For>(0)
    ) {
        reading(args.clone());
    }
}
