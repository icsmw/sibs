use crate::{
    elements::{Element, ElementRef, SimpleString, Task},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Task {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(
                Element::arbitrary_with((vec![ElementRef::VariableDeclaration], deep)),
                0..=5,
            ),
            prop::collection::vec(
                Element::arbitrary_with((vec![ElementRef::Reference], deep)),
                0..=5,
            ),
            Element::arbitrary_with((vec![ElementRef::Block], deep)),
            "[a-zA-Z_]*".prop_map(String::from),
        )
            .prop_map(|(declarations, dependencies, block, name)| Task {
                declarations,
                block: Box::new(block),
                token: 0,
                dependencies,
                name: SimpleString {
                    value: name,
                    token: 0,
                },
            })
            .boxed()
    }
}

fn reading(task: Task) {
    get_rt().block_on(async {
        let origin = format!("{task};");
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
        args in any_with::<Task>(0)
    ) {
        reading(args.clone());
    }
}
