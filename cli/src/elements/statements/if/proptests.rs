use crate::{
    elements::{task::Task, If, IfThread},
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

impl Arbitrary for If {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(IfThread::arbitrary_with((0, deep)), 1..=3),
            prop::collection::vec(IfThread::arbitrary_with((1, deep)), 1..=1),
        )
            .prop_map(|(ifs, elses)| If {
                threads: [ifs, elses].concat(),
                token: 0,
            })
            .boxed()
    }
}

fn reading(if_block: If) {
    get_rt().block_on(async {
        let origin = format!("@test {{\n{if_block};\n}};");
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
        args in any_with::<If>(0)
    ) {
        reading(args.clone());
    }
}
