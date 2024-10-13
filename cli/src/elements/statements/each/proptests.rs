use crate::{
    elements::{task::Task, Each, Element, ElementId},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Each {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((vec![ElementId::Block], deep)),
            Element::arbitrary_with((vec![ElementId::VariableName], deep)),
            Element::arbitrary_with((vec![ElementId::VariableName], deep)),
        )
            .prop_map(|(block, variable, input)| Each {
                block: Box::new(block),
                variable: Box::new(variable),
                input: Box::new(input),
                token: 0,
            })
            .boxed()
    }
}

fn reading(each: Each) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{each};\n}};");
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
        args in any_with::<Each>(0)
    ) {
        reading(args.clone());
    }
}
