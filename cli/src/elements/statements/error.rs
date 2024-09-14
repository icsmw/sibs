use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope, TokenGetter,
        TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub token: usize,
    pub output: Box<Element>,
}

impl TryDissect<Error> for Error {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Error>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Error);
        if reader.move_to().word(&[words::ERROR]).is_none() {
            return Ok(None);
        };
        let output = Element::include(reader, &[ElTarget::PatternString])?
            .ok_or(E::NoErrorMessageDefinition.by_reader(reader))?;
        Ok(Some(Error {
            token: close(reader),
            output: Box::new(output),
        }))
    }
}

impl Dissect<Error, Error> for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", words::ERROR, self.output)
    }
}

impl Formation for Error {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{} {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            words::ERROR,
            self.output.format(cursor)
        )
    }
}

impl TokenGetter for Error {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Error {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { self.output.verification(owner, components, prev, cx).await })
    }
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move { self.output.linking(owner, components, prev, cx).await })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { Ok(ValueRef::Empty) })
    }
}

impl TryExecute for Error {
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Element>,
        _components: &'a [Element],
        _args: &'a [Value],
        _prev: &'a Option<PrevValue>,
        _cx: Context,
        sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move { todo!("not implemented") })
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{ElTarget, Element, Error, Metadata, Return, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;
    use tokio_util::sync::CancellationToken;

    impl Arbitrary for Error {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((vec![ElTarget::PatternString], deep))
                .prop_map(|output| Error {
                    output: Box::new(output),
                    token: 0,
                })
                .boxed()
        }
    }
    fn reading(err: Error) {
        get_rt().block_on(async {
            let ret = Return {
                token: 0,
                output: Some(Box::new(Element::Error(err, Metadata::empty()))),
                signal: CancellationToken::new(),
            };
            let origin = format!("@test {{\n{ret};\n}};");
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
            args in any_with::<Error>(0)
        ) {
            reading(args.clone());
        }
    }
}
