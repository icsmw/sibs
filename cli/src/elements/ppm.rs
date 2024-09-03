use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, GlobalVariablesMap, LinkingResult, Scope, TokenGetter,
        TryExecute, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum PpmCall {
    Function(Box<Element>),
    VectorElementAccessor(Box<Element>),
}

impl fmt::Display for PpmCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function(el) => format!(".{el}"),
                Self::VectorElementAccessor(n) => format!("[{n}]"),
            }
        )
    }
}

/// PPM - Post-processing Method
#[derive(Debug, Clone)]
pub struct Ppm {
    pub token: usize,
    pub call: PpmCall,
}

impl TryDissect<Ppm> for Ppm {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Ppm);
        let call = if reader.move_to().char(&[&chars::DOT]).is_some() {
            let Some(el) = Element::include(reader, &[ElTarget::Function])? else {
                return Err(E::NoCallFunction.linked(&close(reader)));
            };
            PpmCall::Function(Box::new(el))
        } else if reader
            .group()
            .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            let Some(el) = Element::include(
                &mut inner,
                &[
                    ElTarget::Integer,
                    ElTarget::Function,
                    ElTarget::VariableName,
                ],
            )?
            else {
                return Err(E::NoElementAccessor.linked(&close(reader)));
            };
            PpmCall::VectorElementAccessor(Box::new(el))
        } else {
            return Ok(None);
        };
        Ok(Some(Ppm {
            token: close(reader),
            call,
        }))
    }
}

impl Dissect<Ppm, Ppm> for Ppm {}

impl fmt::Display for Ppm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.call)
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

impl ExpectedValueType for Ppm {
    fn varification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }
    fn linking<'a>(
        &'a self,
        _variables: &'a mut GlobalVariablesMap,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move { Ok(()) })
    }
    fn expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { Ok(ValueRef::Empty) })
    }
}

impl TryExecute for Ppm {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<Value>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let Some(value) = prev else {
                return Err(operator::E::CallPPMWithoutPrevValue.linked(&self.token));
            };
            Ok(match &self.call {
                PpmCall::Function(func) => {
                    func.execute(owner, components, args, prev, cx, sc, token)
                        .await?
                }
                PpmCall::VectorElementAccessor(el) => {
                    let n = el
                        .execute(owner, components, args, prev, cx, sc, token)
                        .await?
                        .as_num()
                        .ok_or(operator::E::FailToExtractAccessorIndex.by(&**el))?;
                    if n < 0 {
                        return Err(operator::E::NegativeAccessorIndex(n).by(&**el));
                    }
                    let n = n as usize;
                    match value {
                        Value::String(v) => Value::String(
                            v.chars()
                                .nth(n)
                                .ok_or(
                                    operator::E::OutOfBounds(v.chars().count(), n)
                                        .linked(&self.token),
                                )?
                                .to_string(),
                        ),
                        Value::Vec(v) => v
                            .get(n)
                            .map(|v| v.duplicate())
                            .ok_or(operator::E::OutOfBounds(v.len(), n).linked(&self.token))?,
                        _ => Err(operator::E::AccessByIndexNotSupported(value.to_string())
                            .linked(&self.token))?,
                    }
                }
            })
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
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Function]))?
                {
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
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Function]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    match el {
                        Element::Function(el, md) => {
                            assert_eq!(
                                trim_carets(&format!("{el}")),
                                reader.get_fragment(&el.token())?.content
                            );
                            let ppm = md.ppm.as_ref().expect("Ppm function should be present");
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
        elements::{ElTarget, Element, Ppm, PpmCall, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for PpmCall {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((
                vec![
                    ElTarget::Function,
                    ElTarget::VariableName,
                    ElTarget::Integer,
                ],
                deep,
            ))
            .prop_map(|el| {
                if matches!(el, Element::Function(..)) {
                    PpmCall::Function(Box::new(el))
                } else {
                    PpmCall::VectorElementAccessor(Box::new(el))
                }
            })
            .boxed()
        }
    }
    impl Arbitrary for Ppm {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            PpmCall::arbitrary_with(deep)
                .prop_map(|call| Ppm { token: 0, call })
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
