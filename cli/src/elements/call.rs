use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element, Gatekeeper},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, Formation, FormationCursor, Scope,
        TokenGetter, TryExecute, Value,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Call {
    pub token: usize,
    pub func: Box<Element>,
}

impl TryDissect<Call> for Call {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Call);
        if reader.move_to().char(&[&chars::DOT]).is_none() {
            return Ok(None);
        }
        let Some(el) = Element::include(reader, &[ElTarget::Function])? else {
            return Err(E::NoCallFunction.linked(&close(reader)));
        };
        Ok(Some(Call {
            token: close(reader),
            func: Box::new(el),
        }))
    }
}

impl Dissect<Call, Call> for Call {}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.func)
    }
}

impl Formation for Call {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl TokenGetter for Call {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExecute for Call {
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

impl Execute for Call {}

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
        let content = include_str!("../tests/reading/calls.sibs");
        let len = content.split('\n').count();
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/calls.sibs"),
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
        let content = include_str!("../tests/reading/calls.sibs");
        let len = content.split('\n').count();
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/calls.sibs"),
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
                            let call = md.call.as_ref().expect("Call function should be present");
                            assert_eq!(
                                trim_carets(&format!(".{call}")),
                                reader.get_fragment(&call.token())?.content
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

    // #[tokio::test]
    // async fn error() {
    //     let samples = include_str!("../tests/error/function.sibs");
    //     let samples = samples.split('\n').collect::<Vec<&str>>();
    //     let mut count = 0;
    //     let cfg = Configuration::logs(false);
    //     for sample in samples.iter() {
    //         count += read_string!(&cfg, sample, |reader: &mut Reader, _: &mut Sources| {
    //             let func = Function::dissect(reader);
    //             if func.is_ok() {
    //                 let _ = reader.move_to().char(&[&chars::SEMICOLON]);
    //                 assert!(
    //                     !reader.rest().trim().is_empty(),
    //                     "Line {}: func: {:?}",
    //                     count + 1,
    //                     func
    //                 );
    //             } else {
    //                 assert!(func.is_err(), "Line {}: func: {:?}", count + 1, func);
    //             }
    //             Ok::<usize, LinkedErr<E>>(1)
    //         });
    //     }
    //     assert_eq!(count, samples.len());
    // }
}
