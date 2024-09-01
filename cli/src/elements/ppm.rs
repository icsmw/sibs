use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, LinkingResult, Scope, TokenGetter, TryExecute, Value,
        ValueRef, VerificationResult,
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
        _owner: &'a Component,
        _components: &'a [Component],
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }
    fn linking<'a>(
        &'a self,
        _variables: &'a mut GlobalVariablesMap,
        _owner: &'a Component,
        _components: &'a [Component],
        _cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move { Ok(()) })
    }
    fn expected<'a>(
        &'a self,
        _owner: &'a Component,
        _components: &'a [Component],
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { Ok(ValueRef::Empty) })
    }
}
impl TryExecute for Ppm {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [Value],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move { Ok(None) })
    }
}

impl Execute for Ppm {}

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
