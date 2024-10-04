use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope, TokenGetter,
        TryExecute, TryExpectedValueType, Value, VerificationResult,
    },
    reader::{Dissect, Reader, TryDissect, E},
};
use std::fmt;

/// PPM - Post-processing Method
#[derive(Debug, Clone)]
pub struct Ppm {
    pub token: usize,
    pub el: Box<Element>,
}

impl Ppm {
    pub fn is_call(&self) -> bool {
        matches!(*self.el, Element::Call(..))
    }
}

impl TryDissect<Ppm> for Ppm {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Ppm);
        Ok(
            Element::include(reader, &[ElTarget::Call, ElTarget::Accessor])?.map(|el| Ppm {
                token: close(reader),
                el: Box::new(el),
            }),
        )
    }
}

impl Dissect<Ppm, Ppm> for Ppm {}

impl fmt::Display for Ppm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.el)
    }
}

impl Formation for Ppm {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl TokenGetter for Ppm {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Ppm {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { self.el.verification(owner, components, prev, cx).await })
    }
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { self.el.linking(owner, components, prev, cx).await })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { self.el.expected(owner, components, prev, cx).await })
    }
}

impl TryExecute for Ppm {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            self.el
                .execute(owner, components, args, prev, cx, sc, token)
                .await
        })
    }
}

#[cfg(test)]
mod reading {

    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{chars, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../tests/reading/ppm.sibs");
        let len = content.split('\n').count();
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/ppm.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) = src.report_err_if(Element::include(
                    reader,
                    &[ElTarget::Function, ElTarget::VariableName],
                ))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{el};")),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, len);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        let content = include_str!("../tests/reading/ppm.sibs");
        let len = content.split('\n').count();
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/ppm.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) = src.report_err_if(Element::include(
                    reader,
                    &[ElTarget::Function, ElTarget::VariableName],
                ))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    match el {
                        Element::Function(el, md) => {
                            assert_eq!(
                                trim_carets(&format!("{el}")),
                                reader.get_fragment(&el.token())?.content
                            );
                            let ppm = md.ppm.as_ref().expect("Ppm element should be present");
                            assert_eq!(
                                trim_carets(&format!("{ppm}")),
                                reader.get_fragment(&ppm.token())?.content
                            );
                        }
                        Element::VariableName(el, md) => {
                            assert_eq!(
                                trim_carets(&format!("{el}")),
                                reader.get_fragment(&el.token())?.content
                            );
                            let ppm = md.ppm.as_ref().expect("Ppm element should be present");
                            assert_eq!(
                                trim_carets(&format!("{ppm}")),
                                reader.get_fragment(&ppm.token())?.content
                            );
                        }
                        _ => {
                            panic!("Not considered element: {el:?}")
                        }
                    }
                    count += 1;
                }
                assert_eq!(count, len);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{ElTarget, Element, Ppm, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Ppm {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((vec![ElTarget::Call, ElTarget::Accessor], deep))
                .prop_map(|el| Ppm {
                    el: Box::new(el),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(ppm: Ppm) {
        get_rt().block_on(async {
            let origin = format!("@test {{\nsome_initial_func(){ppm};\n}};");
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
            args in any_with::<Ppm>(2)
        ) {
            reading(args.clone());
        }
    }
}
