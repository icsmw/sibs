use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope, TokenGetter,
        TryExecute, TryExpectedValueType, Value, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct For {
    pub index: Box<Element>,
    pub target: Box<Element>,
    pub block: Box<Element>,
    pub token: usize,
}

impl TryDissect<For> for For {
    fn try_dissect(reader: &mut Reader) -> Result<Option<For>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::For);
        if reader.move_to().word(&[words::FOR]).is_some() {
            let Some(index) = Element::include(reader, &[ElTarget::VariableName])? else {
                return Err(E::NoIndexInForLoop.by_reader(reader));
            };
            if reader.move_to().word(&[words::IN]).is_none() {
                return Err(E::NoINKeywordInForLoop.by_reader(reader));
            }
            let Some(target) = Element::include(
                reader,
                &[ElTarget::Range, ElTarget::VariableName, ElTarget::Values],
            )?
            else {
                return Err(E::NoRangeInForLoop.by_reader(reader));
            };
            let Some(mut block) = Element::include(reader, &[ElTarget::Block])? else {
                return Err(E::NoBodyInForLoop.by_reader(reader));
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElTarget::For);
            }
            Ok(Some(Self {
                index: Box::new(index),
                target: Box::new(target),
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<For, For> for For {}

impl fmt::Display for For {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "for {} in {} {}", self.index, self.target, self.block)
    }
}

impl Formation for For {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::For));
        format!(
            "{}for {} in {} {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.index,
            self.target,
            self.block.format(&mut inner)
        )
    }
}

impl TokenGetter for For {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for For {
    fn try_varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move {
            self.index.varification(owner, components, prev, cx).await?;
            self.target
                .varification(owner, components, prev, cx)
                .await?;
            self.block.varification(owner, components, prev, cx).await
        })
    }
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            self.index.linking(owner, components, prev, cx).await?;
            self.target.linking(owner, components, prev, cx).await?;
            self.block.linking(owner, components, prev, cx).await
        })
    }

    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { self.block.expected(owner, components, prev, cx).await })
    }
}

impl TryExecute for For {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            self.block
                .execute(owner, components, args, prev, cx, sc, token)
                .await
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::For,
        error::LinkedErr,
        inf::{tests::*, Configuration, TokenGetter},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/for.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(For::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 5);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/for.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(For::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.block.to_string()),
                        trim_carets(&reader.get_fragment(&entity.block.token())?.lined),
                    );
                    count += 1;
                }
                assert_eq!(count, 5);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    // #[tokio::test]
    // async fn error() {
    //     let samples = include_str!("../../tests/error/first.sibs");
    //     let samples = samples.split('\n').collect::<Vec<&str>>();
    //     let mut count = 0;
    //     for sample in samples.iter() {
    //         count += read_string!(
    //             &Configuration::logs(false),
    //             sample,
    //             |reader: &mut Reader, _: &mut Sources| {
    //                 assert!(For::dissect(reader).is_err());
    //                 Ok::<usize, LinkedErr<E>>(1)
    //             }
    //         );
    //     }
    //     assert_eq!(count, samples.len());
    // }
}

// #[cfg(test)]
// mod processing {
//     use tokio_util::sync::CancellationToken;

//     use crate::{
//         elements::{ElTarget, Element},
//         error::LinkedErr,
//         inf::{
//             operator::{Execute, E},
//             Configuration, Context, Journal, Scope,
//         },
//         process_string,
//         reader::{chars, Reader, Sources},
//     };

//     #[tokio::test]
//     async fn reading() {
//         process_string!(
//             &Configuration::logs(false),
//             &include_str!("../../tests/processing/first.sibs"),
//             |reader: &mut Reader, src: &mut Sources| {
//                 let mut tasks: Vec<Element> = Vec::new();
//                 while let Some(task) =
//                     src.report_err_if(Element::include(reader, &[ElTarget::Task]))?
//                 {
//                     let _ = reader.move_to().char(&[&chars::SEMICOLON]);
//                     tasks.push(task);
//                 }
//                 Ok::<Vec<Element>, LinkedErr<E>>(tasks)
//             },
//             |tasks: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
//                 for task in tasks.iter() {
//                     let result = task
//                         .execute(
//                             None,
//                             &[],
//                             &[],
//                             &None,
//                             cx.clone(),
//                             sc.clone(),
//                             CancellationToken::new(),
//                         )
//                         .await?;
//                     assert_eq!(
//                         result.as_string().expect("Task returns string value"),
//                         "true".to_owned()
//                     );
//                 }
//                 Ok::<(), LinkedErr<E>>(())
//             }
//         );
//     }
// }

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{ElTarget, Element, For, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for For {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((vec![ElTarget::VariableName], deep)),
                Element::arbitrary_with((
                    vec![ElTarget::Range, ElTarget::VariableName, ElTarget::Values],
                    deep,
                )),
                Element::arbitrary_with((vec![ElTarget::Block], deep)),
            )
                .prop_map(|(index, target, block)| For {
                    index: Box::new(index),
                    target: Box::new(target),
                    block: Box::new(block),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(instance: For) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n{instance};\n}};");
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
            args in any_with::<For>(0)
        ) {
            reading(args.clone());
        }
    }
}
