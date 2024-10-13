use crate::elements::{Closure, Element, ElementId};
use proptest::prelude::*;
use uuid::Uuid;

impl Arbitrary for Closure {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((vec![ElementId::Block], deep)),
            prop::collection::vec(
                Element::arbitrary_with((vec![ElementId::VariableName], deep)),
                0..=3,
            ),
        )
            .prop_map(|(block, args)| Closure {
                args,
                token: 0,
                block: Box::new(block),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}

// fn reading(func: Closure) {
//     get_rt().block_on(async {
//         let origin = format!("@test {{\n{func};\n}};");
//         read_string!(
//             &Configuration::logs(false),
//             &origin,
//             |reader: &mut Reader, src: &mut Sources| {
//                 let task = src
//                     .report_err_if(Task::dissect(reader))?
//                     .expect("Task read");
//                 assert_eq!(format!("{task};"), origin);
//                 Ok::<(), LinkedErr<E>>(())
//             }
//         );
//     })
// }

// proptest! {
//     #![proptest_config(ProptestConfig {
//         max_shrink_iters: 5000,
//         ..ProptestConfig::with_cases(10)
//     })]
//     #[test]
//     fn test_run_task(
//         args in any_with::<Closure>(0)
//     ) {
//         reading(args.clone());
//     }
// }
