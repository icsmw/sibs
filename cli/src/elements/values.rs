use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope,
        TokenGetter, TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Values {
    pub elements: Vec<Element>,
    pub token: usize,
}

impl TryDissect<Values> for Values {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Values>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Values);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Ok(None);
        }
        let token = reader.token()?;
        let mut inner = token.bound;
        let mut elements: Vec<Element> = Vec::new();
        if inner.rest().trim().is_empty() {
            return Ok(Some(Values {
                token: close(reader),
                elements,
            }));
        }
        while let Some(el) = Element::include(
            &mut inner,
            &[
                ElTarget::Command,
                ElTarget::Function,
                ElTarget::If,
                ElTarget::PatternString,
                ElTarget::Reference,
                ElTarget::Values,
                ElTarget::Comparing,
                ElTarget::VariableName,
                ElTarget::Integer,
                ElTarget::Boolean,
            ],
        )? {
            if inner.move_to().char(&[&chars::COMMA]).is_none() && !inner.rest().trim().is_empty() {
                Err(E::MissedComma.by_reader(&inner))?;
            }
            elements.push(el);
        }
        if !inner.rest().trim().is_empty() {
            if let Some((content, _)) = inner.until().char(&[&chars::COMMA]) {
                Err(E::UnrecognizedCode(content).by_reader(&inner))?;
            } else {
                Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
            }
        }
        Ok(Some(Values {
            token: close(reader),
            elements,
        }))
    }
}

impl Dissect<Values, Values> for Values {}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.elements
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Formation for Values {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        if self.to_string().len() > cursor.max_len() && self.elements.len() > cursor.max_items() {
            format!(
                "{}(\n{}\n{})",
                cursor.offset_as_string_if(&[ElTarget::Block]),
                self.elements
                    .iter()
                    .map(|v| format!(
                        "{}{}",
                        cursor.right().offset_as_string(),
                        v.format(&mut cursor.reown(Some(ElTarget::Values)).right())
                    ))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                cursor.offset_as_string_if(&[ElTarget::Block, ElTarget::Function])
            )
        } else {
            format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
        }
    }
}

impl TokenGetter for Values {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Values {
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
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            let mut ty: Option<_> = None;
            for el in self.elements.iter() {
                if let Some(ty) = ty.as_ref() {
                    let current = el.expected(owner, components, prev, cx).await?;
                    if !current.is_compatible(ty) {
                        return Err(operator::E::DismatchTypesInVector(
                            ty.to_string(),
                            current.to_string(),
                        )
                        .by(el));
                    }
                } else {
                    ty = Some(el.expected(owner, components, prev, cx).await?)
                }
            }
            Ok(ValueRef::Vec(Box::new(ty.ok_or(operator::E::EmptyVector)?)))
        })
    }
}

impl TryExecute for Values {
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
            let mut values: Vec<Value> = Vec::new();
            for el in self.elements.iter() {
                values.push(
                    el.execute(
                        owner,
                        components,
                        args,
                        prev,
                        cx.clone(),
                        sc.clone(),
                        token.clone(),
                    )
                    .await?,
                );
            }
            Ok(Value::Vec(values))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Values,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let samples = include_str!("../tests/reading/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                let entity = src.report_err_if(Values::dissect(reader))?;
                assert!(entity.is_some(), "Line: {}", count + 1);
                let entity = entity.unwrap();
                assert_eq!(
                    trim_carets(reader.recent()),
                    trim_carets(&format!("{entity}")),
                    "Line: {}",
                    count + 1
                );
                count += 1;
                Ok::<usize, LinkedErr<E>>(count)
            });
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn tokens() {
        let samples = include_str!("../tests/reading/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                let entity = src.report_err_if(Values::dissect(reader))?.unwrap();
                assert_eq!(
                    trim_carets(&entity.to_string()),
                    reader.get_fragment(&entity.token)?.lined,
                    "Line: {}",
                    count + 1
                );
                for el in entity.elements.iter() {
                    assert_eq!(
                        trim_carets(&el.to_string()),
                        trim_carets(&reader.get_fragment(&el.token())?.content),
                        "Line: {}",
                        count + 1
                    );
                }
                count += 1;
                Ok::<usize, LinkedErr<E>>(count)
            });
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../tests/error/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, _: &mut Sources| {
                let entity = Values::dissect(reader);
                assert!(entity.is_err());
                Ok::<usize, LinkedErr<E>>(1)
            });
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, Journal, Scope, Value,
        },
        process_string, read_string,
        reader::{chars, Reader, Sources},
    };

    const VALUES: &[(&str, &str)] = &[
        ("a0", "a"),
        ("a1", "a,b"),
        ("a2", "a,b,c"),
        ("a3", "a,b,c"),
        ("a4", "aa,bb,cc"),
        ("a5", "a:a,b:b"),
    ];
    const NESTED_VALUES: &[(&str, &str)] = &[("a6", "c:a,d:b,d:c")];

    #[tokio::test]
    async fn reading() {
        let components: Vec<Element> = read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/values_components.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Component]))?
                {
                    components.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(components)
            }
        );
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/values.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for task in tasks.iter() {
                    task.execute(
                        components.first(),
                        &components,
                        &[],
                        &None,
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new(),
                    )
                    .await?;
                }
                for (name, value) in VALUES.iter() {
                    assert_eq!(
                        sc.get_var(name)
                            .await?
                            .unwrap()
                            .as_strings()
                            .unwrap()
                            .join(","),
                        value.to_string()
                    );
                }
                for (name, value) in NESTED_VALUES.iter() {
                    let stored = sc.get_var(name).await?.unwrap();
                    let values = stored.get::<Vec<Value>>().unwrap();
                    let mut output: Vec<String> = Vec::new();
                    for value in values.iter() {
                        output = [output, value.as_strings().unwrap()].concat();
                    }
                    assert_eq!(output.join(","), value.to_string());
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::values::{ElTarget, Element, Values},
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Values {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            let max = 5;
            prop::collection::vec(
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElTarget::VariableName, ElTarget::Integer, ElTarget::Boolean]
                    } else {
                        vec![
                            ElTarget::Command,
                            ElTarget::Function,
                            ElTarget::If,
                            ElTarget::PatternString,
                            ElTarget::Reference,
                            ElTarget::Values,
                            ElTarget::Comparing,
                            ElTarget::VariableName,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ]
                    },
                    deep,
                )),
                1..max,
            )
            .prop_map(|elements| Values { elements, token: 0 })
            .boxed()
        }
    }
}
