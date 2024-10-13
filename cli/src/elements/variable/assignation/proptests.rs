use crate::{
    elements::{Element, ElementId, Task, VariableAssignation},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for VariableAssignation {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementId::Function,
                        ElementId::PatternString,
                        ElementId::Values,
                        ElementId::Command,
                        ElementId::VariableName,
                        ElementId::Integer,
                        ElementId::Boolean,
                    ]
                } else {
                    vec![
                        ElementId::Block,
                        ElementId::First,
                        ElementId::Function,
                        ElementId::If,
                        ElementId::PatternString,
                        ElementId::Values,
                        ElementId::Comparing,
                        ElementId::Command,
                        ElementId::VariableName,
                        ElementId::Integer,
                        ElementId::Boolean,
                        ElementId::Reference,
                        ElementId::Compute,
                    ]
                },
                deep,
            )),
            Element::arbitrary_with((vec![ElementId::VariableName], deep)),
            prop_oneof![Just(true), Just(false),].boxed(),
        )
            .prop_map(move |(assignation, variable, global)| VariableAssignation {
                assignation: Box::new(assignation),
                global,
                variable: Box::new(variable),
                token: 0,
            })
            .boxed()
    }
}

fn reading(assignation: VariableAssignation) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{assignation};\n}};");
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
        args in any_with::<VariableAssignation>(0)
    ) {
        reading(args.clone());
    }
}
