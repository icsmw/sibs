use crate::{
    elements::{compute::Operator, Compute, Element, ElementRef, Task},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Compute {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![ElementRef::VariableName, ElementRef::Integer]
                } else {
                    vec![
                        ElementRef::Function,
                        ElementRef::VariableName,
                        ElementRef::Integer,
                    ]
                },
                deep,
            )),
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![ElementRef::VariableName, ElementRef::Integer]
                } else {
                    vec![
                        ElementRef::Function,
                        ElementRef::VariableName,
                        ElementRef::Integer,
                    ]
                },
                deep,
            )),
            prop_oneof![
                Just(Operator::Div),
                Just(Operator::Inc),
                Just(Operator::Dec),
                Just(Operator::Mlt)
            ]
            .boxed(),
        )
            .prop_map(move |(left, right, operator)| Compute {
                left: Box::new(left),
                operator,
                right: Box::new(right),
                token: 0,
            })
            .boxed()
    }
}

fn reading(compute: Compute) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n$var = {compute};\n}};");
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
        args in any_with::<Compute>(0)
    ) {
        reading(args.clone());
    }
}
