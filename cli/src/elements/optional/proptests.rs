use crate::{
    elements::{Element, ElementId, Optional, Task},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Optional {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![ElementId::VariableName, ElementId::Reference]
                } else {
                    vec![
                        ElementId::Function,
                        ElementId::VariableName,
                        ElementId::Reference,
                        ElementId::Block,
                        ElementId::Comparing,
                    ]
                },
                deep,
            )),
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementId::Function,
                        ElementId::Reference,
                        ElementId::VariableName,
                        ElementId::Integer,
                        ElementId::Boolean,
                    ]
                } else {
                    vec![
                        ElementId::Function,
                        ElementId::Reference,
                        ElementId::VariableAssignation,
                        ElementId::VariableName,
                        ElementId::Block,
                        ElementId::Each,
                        ElementId::First,
                        ElementId::PatternString,
                        ElementId::Command,
                        ElementId::Integer,
                        ElementId::Boolean,
                    ]
                },
                deep,
            )),
        )
            .prop_map(|(condition, action)| Optional {
                condition: Box::new(condition),
                action: Box::new(action),
                token: 0,
            })
            .boxed()
    }
}

fn reading(optional: Optional) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{optional};\n}};");
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
        args in any_with::<Optional>(0)
    ) {
        reading(args.clone());
    }
}
