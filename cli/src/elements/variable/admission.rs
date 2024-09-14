use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, ExecutePinnedResult, ExpectedResult, Formation, FormationCursor,
        LinkingResult, PrevValue, PrevValueExpectation, Scope, TokenGetter, TryExecute,
        TryExpectedValueType, Value, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableName {
    pub name: String,
    pub token: usize,
}

impl TryDissect<VariableName> for VariableName {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableName>, LinkedErr<E>> {
        reader.move_to().any();
        let close = reader.open_token(ElTarget::VariableName);
        if reader.move_to().char(&[&chars::DOLLAR]).is_some() {
            let content = reader
                .until()
                .char(&[
                    &chars::COLON,
                    &chars::WS,
                    &chars::EQUAL,
                    &chars::SEMICOLON,
                    &chars::COMMA,
                    &chars::OPEN_SQ_BRACKET,
                    &chars::DOT,
                ])
                .map(|(content, _char)| content)
                .unwrap_or_else(|| reader.move_to().end());
            Ok(Some(VariableName::new(content, close(reader))?))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<VariableName, VariableName> for VariableName {}

impl VariableName {
    pub fn new(mut name: String, token: usize) -> Result<Self, LinkedErr<E>> {
        name = name.trim().to_string();
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
            || name.is_empty()
        {
            Err(E::InvalidVariableName.linked(&token))
        } else {
            Ok(Self { name, token })
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl TokenGetter for VariableName {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for VariableName {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }

    fn try_linking<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move { Ok(()) })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            cx.variables
                .get(&owner.as_component()?.uuid, &self.name)
                .await
                .map_err(|e| LinkedErr::new(e, Some(self.token())))
        })
    }
}

impl TryExecute for VariableName {
    fn try_execute<'a>(
        &'a self,
        _: Option<&'a Element>,
        _: &'a [Element],
        _: &'a [Value],
        _: &'a Option<PrevValue>,
        _: Context,
        sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            Ok(sc
                .get_var(&self.name)
                .await?
                .ok_or(operator::E::VariableIsNotAssigned(self.name.to_owned()))?
                .duplicate())
        })
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.name)
    }
}

impl Formation for VariableName {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{self}", cursor.offset_as_string_if(&[ElTarget::Block]))
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::VariableName,
        error::LinkedErr,
        inf::Configuration,
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let samples = include_str!("../../tests/reading/variable_name.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, src: &mut Sources| {
                    src.report_err_if(VariableName::dissect(reader))?.unwrap();
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn tokens() {
        let samples = include_str!("../../tests/reading/variable_name.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, src: &mut Sources| {
                    let variable_name = src.report_err_if(VariableName::dissect(reader))?.unwrap();
                    let fragment = reader.get_fragment(&reader.token()?.id)?.content;
                    assert_eq!(format!("${}", variable_name.name), fragment);
                    assert_eq!(fragment, variable_name.to_string());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/variable_name.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(VariableName::dissect(reader).is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod proptest {
    use crate::{elements::variable::VariableName, reader::words};
    use proptest::prelude::*;

    impl Arbitrary for VariableName {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            "[a-z][a-z0-9]*"
                .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
                .prop_map(String::from)
                .prop_map(|name| VariableName {
                    name: if name.is_empty() {
                        "min".to_owned()
                    } else {
                        name
                    },
                    token: 0,
                })
                .boxed()
        }
    }
}
