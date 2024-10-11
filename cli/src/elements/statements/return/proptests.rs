use crate::{
    elements::{Element, ElementRef, Return, Task},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Return {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        Element::arbitrary_with((
            if deep > MAX_DEEP {
                vec![
                    ElementRef::VariableName,
                    ElementRef::Error,
                    ElementRef::Integer,
                    ElementRef::Boolean,
                ]
            } else {
                vec![
                    ElementRef::Values,
                    ElementRef::VariableName,
                    ElementRef::Error,
                    ElementRef::Function,
                    ElementRef::If,
                    ElementRef::Integer,
                    ElementRef::Boolean,
                    ElementRef::PatternString,
                ]
            },
            deep,
        ))
        .prop_map(|output| Return {
            output: Some(Box::new(output)),
            token: 0,
        })
        .boxed()
    }
}

fn reading(ret: Return) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{ret};\n}};");
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
        args in any_with::<Return>(0)
    ) {
        reading(args.clone());
    }
}
