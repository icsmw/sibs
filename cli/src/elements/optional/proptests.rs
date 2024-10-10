use crate::{
    elements::{Element, ElementRef, Optional, Task},
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
                    vec![ElementRef::VariableName, ElementRef::Reference]
                } else {
                    vec![
                        ElementRef::Function,
                        ElementRef::VariableName,
                        ElementRef::Reference,
                        ElementRef::Block,
                        ElementRef::Comparing,
                    ]
                },
                deep,
            )),
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementRef::Function,
                        ElementRef::Reference,
                        ElementRef::VariableName,
                        ElementRef::Integer,
                        ElementRef::Boolean,
                    ]
                } else {
                    vec![
                        ElementRef::Function,
                        ElementRef::Reference,
                        ElementRef::VariableAssignation,
                        ElementRef::VariableName,
                        ElementRef::Block,
                        ElementRef::Each,
                        ElementRef::First,
                        ElementRef::PatternString,
                        ElementRef::Command,
                        ElementRef::Integer,
                        ElementRef::Boolean,
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
