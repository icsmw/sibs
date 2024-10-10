use crate::{
    elements::{Block, Element, ElementRef, Task},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Block {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementRef::Function,
                        ElementRef::VariableAssignation,
                        ElementRef::Optional,
                        ElementRef::Command,
                        ElementRef::PatternString,
                        ElementRef::Reference,
                        ElementRef::Boolean,
                        ElementRef::Integer,
                    ]
                } else {
                    vec![
                        ElementRef::Function,
                        ElementRef::VariableAssignation,
                        ElementRef::If,
                        ElementRef::Optional,
                        ElementRef::First,
                        ElementRef::Breaker,
                        ElementRef::Each,
                        ElementRef::Join,
                        ElementRef::Command,
                        ElementRef::PatternString,
                        ElementRef::Reference,
                        ElementRef::Boolean,
                        ElementRef::Integer,
                        ElementRef::For,
                        ElementRef::Loop,
                        ElementRef::While,
                        ElementRef::Conclusion,
                        ElementRef::VariableName,
                        ElementRef::Values,
                    ]
                },
                deep,
            )),
            1..=10,
        )
        .prop_map(|elements| Block {
            elements,
            owner: None,
            breaker: None,
            token: 0,
        })
        .boxed()
    }
}

fn reading(block: Block) {
    get_rt().block_on(async {
        let origin = format!("@test {block};");
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
        args in any_with::<Block>(0)
    ) {
        reading(args.clone());
    }
}
