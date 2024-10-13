use crate::{
    elements::{Component, Element, ElementId, Gatekeeper},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for Gatekeeper {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((vec![ElementId::Function], deep)),
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![ElementId::Reference]
                } else {
                    // TODO: should be added ElementId::Values with references only
                    vec![ElementId::Reference]
                },
                deep,
            )),
        )
            .prop_map(|(function, action)| Gatekeeper {
                function: Box::new(function),
                refs: Box::new(action),
                token: 0,
            })
            .boxed()
    }
}

fn reading(gatekeeper: Gatekeeper) {
    get_rt().block_on(async {
        let origin = format!("#(test: ./){gatekeeper};\n@test {{\nprint(\"hello world\");\n}};");
        read_string!(
            &Configuration::logs(false),
            &origin,
            |reader: &mut Reader, src: &mut Sources| {
                while let Some(component) = src.report_err_if(Component::dissect(reader))? {
                    assert_eq!(format!("{component}"), origin);
                }
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
        args in any_with::<Gatekeeper>(0)
    ) {
        reading(args.clone());
    }
}
