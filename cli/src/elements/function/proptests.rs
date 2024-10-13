use crate::{
    elements::{Element, ElementId, Function, Task},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{words, Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Function {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > MAX_DEEP {
            ("[a-z][a-z0-9_]*"
                .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
                .prop_map(String::from),)
                .prop_map(|(name,)| Function {
                    args: Vec::new(),
                    token: 0,
                    args_token: 0,
                    name: if name.is_empty() {
                        "min".to_owned()
                    } else {
                        name
                    },
                })
                .boxed()
        } else {
            (
                "[a-z][a-z0-9_]*"
                    .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
                    .prop_map(String::from),
                prop::collection::vec(
                    Element::arbitrary_with((
                        vec![
                            ElementId::Values,
                            ElementId::Function,
                            ElementId::If,
                            ElementId::PatternString,
                            ElementId::Reference,
                            ElementId::Comparing,
                            ElementId::VariableName,
                            ElementId::Command,
                            ElementId::Integer,
                            ElementId::Boolean,
                            ElementId::SimpleString,
                        ],
                        deep,
                    )),
                    0..=3,
                ),
            )
                .prop_map(|(name, args)| Function {
                    args,
                    token: 0,
                    args_token: 0,
                    name: if name.is_empty() {
                        "min".to_owned()
                    } else {
                        name
                    },
                })
                .boxed()
        }
    }
}

fn reading(func: Function) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{func};\n}};");
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
        args in any_with::<Function>(0)
    ) {
        reading(args.clone());
    }
}
