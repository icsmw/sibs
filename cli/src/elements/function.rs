use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, HasOptional, HasRepeated, LinkingResult, PrevValue,
        PrevValueExpectation, Scope, TokenGetter, TryExecute, TryExpectedValueType, Value,
        ValueRef, VerificationResult,
    },
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FuncArg {
    pub value: Value,
    pub token: usize,
}

impl FuncArg {
    pub fn new(value: Value, token: usize) -> Self {
        Self { value, token }
    }
    pub fn err<T: Clone + fmt::Display>(&self, err: T) -> LinkedErr<T> {
        LinkedErr::new(err, Some(self.token))
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Element>,
    pub token: usize,
    pub args_token: usize,
}

impl TryDissect<Function> for Function {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        reader.move_to().any();
        let close = reader.open_token(ElTarget::Function);
        let Some((name, mut stop)) = reader.until().char(&[
            &chars::OPEN_BRACKET,
            &chars::CARET,
            &chars::SEMICOLON,
            &chars::COMMA,
            &chars::WS,
            &chars::OPEN_BRACKET,
            &chars::CLOSE_BRACKET,
        ]) else {
            return Ok(None);
        };
        if stop == chars::WS {
            if reader.move_to().char(&[&chars::OPEN_BRACKET]).is_none() {
                return Ok(None);
            } else {
                let _ = reader.move_to().prev();
                stop = chars::OPEN_BRACKET;
            }
        }
        if stop != chars::OPEN_BRACKET
            || !Reader::is_ascii_alphabetic_and_alphanumeric(
                &name,
                &[&chars::UNDERSCORE, &chars::DASH, &chars::COLON],
            )
            || name.trim().chars().any(|c| c.is_whitespace())
            || name
                .trim()
                .chars()
                .next()
                .map(|c| !c.is_ascii_alphabetic())
                .unwrap_or(true)
            || words::is_reserved(name.trim())
            || name.is_empty()
            || name
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        {
            return Ok(None);
        }
        let args_close = reader.open_token(ElTarget::Function);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Err(E::NotClosedFunctionArgs.by_reader(reader));
        }
        let mut elements: Vec<Element> = Vec::new();
        let mut inner = reader.token()?.bound;
        while !inner.is_empty() {
            if let Some(el) = Element::include(
                &mut inner,
                &[
                    ElTarget::Closure,
                    ElTarget::Values,
                    ElTarget::Function,
                    ElTarget::If,
                    ElTarget::PatternString,
                    ElTarget::Reference,
                    ElTarget::Comparing,
                    ElTarget::VariableName,
                    ElTarget::Command,
                    ElTarget::Integer,
                    ElTarget::Boolean,
                ],
            )? {
                if inner.move_to().char(&[&chars::COMMA]).is_none() && !inner.is_empty() {
                    Err(E::MissedComma.by_reader(&inner))?;
                }
                elements.push(el);
            } else if let Some((content, _)) = inner.until().char(&[&chars::COMMA]) {
                if content.trim().is_empty() {
                    Err(E::NoContentBeforeComma.by_reader(&inner))?;
                }
                elements.push(
                    Element::include(&mut inner.token()?.bound, &[ElTarget::SimpleString])?
                        .ok_or(E::NoContentBeforeComma.by_reader(&inner))?,
                );
                let _ = inner.move_to().char(&[&chars::COMMA]);
            } else if !inner.is_empty() {
                elements.push(
                    Element::include(&mut inner, &[ElTarget::SimpleString])?
                        .ok_or(E::NoContentBeforeComma.by_reader(&inner))?,
                );
            }
        }
        if reader.rest().trim().starts_with(words::DO_ON) {
            return Ok(Some(Self::new(
                close(reader),
                args_close(reader),
                elements,
                name,
            )?));
        }
        Ok(Some(Self::new(
            close(reader),
            args_close(reader),
            elements,
            name,
        )?))
    }
}

impl Dissect<Function, Function> for Function {}

impl Function {
    pub fn new(
        token: usize,
        args_token: usize,
        args: Vec<Element>,
        name: String,
    ) -> Result<Self, LinkedErr<E>> {
        Ok(Self {
            token,
            args_token,
            name,
            args,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_processed_args<'a>(
        &self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> Result<Vec<FuncArg>, operator::E> {
        let mut values: Vec<FuncArg> = Vec::new();
        for arg in self.args.iter() {
            values.push(FuncArg::new(
                arg.execute(
                    owner,
                    components,
                    args,
                    prev,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?,
                arg.token(),
            ))
        }
        if let Some(prev) = prev {
            values.insert(0, FuncArg::new(prev.value.duplicate(), prev.token))
        }
        Ok(values)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

impl Formation for Function {
    fn elements_count(&self) -> usize {
        self.args.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        // fn formated(func: &Function, cursor: &mut FormationCursor) -> String {
        //     format!(
        //         "{}({}{}",
        //         func.name,
        //         func.args
        //             .iter()
        //             .map(|arg| format!(
        //                 "\n{}{}",
        //                 cursor.right().offset_as_string(),
        //                 arg.format(&mut cursor.reown(Some(ElTarget::Function)).right())
        //             ))
        //             .collect::<Vec<String>>()
        //             .join(", "),
        //         if func.args.is_empty() {
        //             ")".to_string()
        //         } else {
        //             format!("\n{})", cursor.offset_as_string_if(&[ElTarget::Block]))
        //         }
        //     )
        // }
        let output = format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElTarget::Block, ElTarget::Component]),
            self
        );
        format!(
            "{output}{}",
            if cursor.parent.is_none() { ";\n" } else { "" }
        )
    }
}

impl TokenGetter for Function {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Function {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move {
            for el in self.args.iter() {
                el.verification(owner, components, prev, cx).await?;
            }
            let desc = cx
                .get_func_desc(&self.name, prev.as_ref().map(|v| v.value.clone()).clone())
                .await?;
            let ex_args = desc.args();
            let mut ac_args = Vec::new();
            for el in self.args.iter() {
                ac_args.push((el.expected(owner, components, prev, cx).await?, el.token()));
            }
            if let Some(prev) = prev {
                ac_args.insert(0, (prev.value.clone(), prev.token));
            }
            if ex_args.has_optional() && ex_args.has_repeated() {
                return Err(operator::E::RepeatedAndOptionalTypes(self.name.to_owned()).by(self));
            }
            if ex_args.has_optional() {
                if ex_args.len() < ac_args.len() {
                    return Err(operator::E::FunctionsArgsNumberNotMatch(
                        self.name.to_owned(),
                        ex_args.len(),
                        ac_args.len(),
                    )
                    .by(self));
                }
                for (n, (actual, actual_token)) in ac_args.iter().enumerate() {
                    if !ex_args[n].is_compatible(actual) {
                        return Err(operator::E::FunctionsArgNotMatchType(
                            self.name.to_owned(),
                            ex_args[n].to_owned(),
                            actual.to_owned(),
                        )
                        .linked(actual_token));
                    }
                }
            } else if ex_args.has_repeated() {
                if ex_args.len() > ac_args.len() {
                    return Err(operator::E::FunctionsArgsNumberNotMatch(
                        self.name.to_owned(),
                        ex_args.len(),
                        ac_args.len(),
                    )
                    .by(self));
                }
                let Some(ValueRef::Repeated(repeated)) = ex_args.last() else {
                    return Err(operator::E::InvalidRepeatedType(self.name.to_owned()).by(self));
                };
                for (n, (actual, actual_token)) in ac_args.iter().enumerate() {
                    if n < ex_args.len() - 1 && !ex_args[n].is_compatible(actual) {
                        return Err(operator::E::FunctionsArgNotMatchType(
                            self.name.to_owned(),
                            ex_args[n].to_owned(),
                            actual.to_owned(),
                        )
                        .linked(actual_token));
                    } else if repeated.is_compatible(actual) {
                        return Err(operator::E::FunctionsArgNotMatchType(
                            self.name.to_owned(),
                            *repeated.clone(),
                            actual.to_owned(),
                        )
                        .linked(actual_token));
                    }
                }
            } else {
                if ex_args.len() != ac_args.len() {
                    return Err(operator::E::FunctionsArgsNumberNotMatch(
                        self.name.to_owned(),
                        ex_args.len(),
                        ac_args.len(),
                    )
                    .by(self));
                }
                for (n, (actual, actual_token)) in ac_args.iter().enumerate() {
                    if !ex_args[n].is_compatible(actual) {
                        return Err(operator::E::FunctionsArgNotMatchType(
                            self.name.to_owned(),
                            ex_args[n].to_owned(),
                            actual.to_owned(),
                        )
                        .linked(actual_token));
                    }
                }
            }
            Ok(())
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
            for el in self.args.iter() {
                el.linking(owner, components, prev, cx).await?;
            }
            Ok(())
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            Ok(cx
                .get_func_desc(&self.name, prev.as_ref().map(|v| v.value.clone()).clone())
                .await?
                .output())
        })
    }
}

impl TryExecute for Function {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        inputs: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let args: Vec<FuncArg> = self
                .get_processed_args(
                    owner,
                    components,
                    inputs,
                    prev,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?;
            Ok(cx
                .execute(
                    &self.name,
                    args,
                    self.args_token,
                    prev.as_ref().map(|v| v.value.clone()).clone(),
                    sc,
                )
                .await?)
        })
    }
}

#[cfg(test)]
mod reading {

    use crate::{
        elements::Function,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../tests/reading/function.sibs");
        let len = content.split('\n').count();
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/function.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Function::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
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
        let content = include_str!("../tests/reading/function.sibs");
        let len = content.split('\n').count();
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/function.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Function::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        reader.get_fragment(&entity.token)?.content
                    );
                    for arg in entity.args.iter() {
                        assert_eq!(
                            trim_carets(&arg.to_string()),
                            reader.get_fragment(&arg.token())?.lined
                        );
                    }
                    count += 1;
                }
                assert_eq!(count, len);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../tests/error/function.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, _: &mut Sources| {
                let func = Function::dissect(reader);
                if func.is_ok() {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert!(
                        !reader.rest().trim().is_empty(),
                        "Line {}: func: {:?}",
                        count + 1,
                        func
                    );
                } else {
                    assert!(func.is_err(), "Line {}: func: {:?}", count + 1, func);
                }
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
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/functions.sibs"),
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
                    let result = task
                        .execute(
                            None,
                            &[],
                            &[],
                            &None,
                            cx.clone(),
                            sc.clone(),
                            CancellationToken::new(),
                        )
                        .await?;
                    assert_eq!(
                        result.as_string().expect("Task returns string value"),
                        "true".to_owned()
                    );
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{ElTarget, Element, Function, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{words, Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Function {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            if deep > MAX_DEEP {
                ("[a-z][a-z0-9_]*"
                    .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
                    .prop_map(String::from),)
                    .prop_map(|(name,)| Function {
                        args: Vec::new(),
                        token: 0,
                        args_token: 0,
                        name: if name.is_empty() {
                            "min".to_owned()
                        } else {
                            name
                        },
                    })
                    .boxed()
            } else {
                (
                    "[a-z][a-z0-9_]*"
                        .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
                        .prop_map(String::from),
                    prop::collection::vec(
                        Element::arbitrary_with((
                            vec![
                                ElTarget::Values,
                                ElTarget::Function,
                                ElTarget::If,
                                ElTarget::PatternString,
                                ElTarget::Reference,
                                ElTarget::Comparing,
                                ElTarget::VariableName,
                                ElTarget::Command,
                                ElTarget::Integer,
                                ElTarget::Boolean,
                                ElTarget::SimpleString,
                            ],
                            deep,
                        )),
                        0..=3,
                    ),
                )
                    .prop_map(|(name, args)| Function {
                        args,
                        token: 0,
                        args_token: 0,
                        name: if name.is_empty() {
                            "min".to_owned()
                        } else {
                            name
                        },
                    })
                    .boxed()
            }
        }
    }

    fn reading(func: Function) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n{func};\n}};");
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
            args in any_with::<Function>(0)
        ) {
            reading(args.clone());
        }
    }
}
