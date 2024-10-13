use crate::{
    elements::{incrementer::Operator, Element, ElementId, Incrementer, Task},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Incrementer {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((vec![ElementId::VariableName], deep)),
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![ElementId::VariableName, ElementId::Integer]
                } else {
                    vec![
                        ElementId::Function,
                        ElementId::VariableName,
                        ElementId::Integer,
                    ]
                },
                deep,
            )),
            prop_oneof![Just(Operator::Inc), Just(Operator::Dec),].boxed(),
        )
            .prop_map(move |(variable, right, operator)| Incrementer {
                variable: Box::new(variable),
                operator,
                right: Box::new(right),
                token: 0,
            })
            .boxed()
    }
}

fn reading(incrementer: Incrementer) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{incrementer};\n}};");
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
        args in any_with::<Incrementer>(0)
    ) {
        reading(args.clone());
    }
}
