use crate::{
    elements::{Element, ElementRef, Join, Metadata, Task, Values},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Join {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![ElementRef::Reference]
                } else {
                    vec![
                        ElementRef::Reference,
                        ElementRef::Function,
                        ElementRef::Command,
                    ]
                },
                deep,
            )),
            1..=10,
        )
        .prop_map(|elements| Values { elements, token: 0 })
        .prop_map(|elements| Join {
            elements: Box::new(Element::Values(elements, Metadata::empty())),
            token: 0,
        })
        .boxed()
    }
}

fn reading(join: Join) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{join};\n}};");
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
        args in any_with::<Join>(0)
    ) {
        reading(args.clone());
    }
}
