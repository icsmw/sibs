use crate::{
    elements::{Component, ElTarget, Element, SimpleString},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Function {
    pub tolerance: bool,
    pub name: String,
    pub args: Vec<Element>,
    pub feed: Option<Box<Function>>,
    pub token: usize,
    pub args_token: usize,
}

impl Reading<Function> for Function {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        reader.move_to().any();
        let close = reader.open_token();
        if reader.move_to().char(&[&chars::AT]).is_some() {
            let (name, ends_with) = reader
                .until()
                .char(&[
                    &chars::CARET,
                    &chars::QUESTION,
                    &chars::SEMICOLON,
                    &chars::WS,
                    &chars::OPEN_BRACKET,
                    &chars::CLOSE_BRACKET,
                ])
                .map(|(str, char)| (str, Some(char)))
                .unwrap_or_else(|| (reader.move_to().end(), None));
            if !Reader::is_ascii_alphabetic_and_alphanumeric(
                &name,
                &[&chars::UNDERSCORE, &chars::DASH, &chars::COLON],
            ) {
                Err(E::InvalidFunctionName(name.to_string()).by_reader(reader))?;
            }
            let args_close = reader.open_token();
            if matches!(
                ends_with,
                Some(chars::SEMICOLON) | Some(chars::CLOSE_BRACKET)
            ) {
                return Ok(Some(Self::new(
                    close(reader),
                    args_close(reader),
                    vec![],
                    name,
                    false,
                )?));
            }
            if ends_with.is_none() {
                return Ok(Some(Self::new(
                    close(reader),
                    args_close(reader),
                    vec![],
                    name,
                    false,
                )?));
            }
            reader.trim();
            let tolerance = if matches!(ends_with, Some(chars::QUESTION)) {
                reader.move_to().next();
                true
            } else {
                false
            };
            let mut elements: Vec<Element> = vec![];
            if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                while !inner.is_empty() {
                    if let Some(el) = Element::include(
                        &mut inner,
                        &[
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
                        if inner.move_to().char(&[&chars::SEMICOLON]).is_none() && !inner.is_empty()
                        {
                            Err(E::MissedSemicolon.by_reader(&inner))?;
                        }
                        elements.push(el);
                    } else if let Some((content, _)) = inner.until().char(&[&chars::SEMICOLON]) {
                        if content.trim().is_empty() {
                            Err(E::NoContentBeforeSemicolon.by_reader(&inner))?;
                        }
                        elements.push(
                            Element::include(&mut inner.token()?.bound, &[ElTarget::SimpleString])?
                                .ok_or(E::NoContentBeforeSemicolon.by_reader(&inner))?,
                        );
                        let _ = inner.move_to().char(&[&chars::SEMICOLON]);
                    } else if !inner.is_empty() {
                        elements.push(
                            Element::include(&mut inner, &[ElTarget::SimpleString])?
                                .ok_or(E::NoContentBeforeSemicolon.by_reader(&inner))?,
                        );
                        // let _ = inner.move_to().char(&[&chars::SEMICOLON]);
                    }
                }
                if elements.is_empty() {
                    Err(E::NoFunctionArguments.by_reader(&inner))?;
                }
            }
            if reader.rest().trim().starts_with(words::DO_ON) {
                return Ok(Some(Self::new(
                    close(reader),
                    args_close(reader),
                    elements,
                    name,
                    tolerance,
                )?));
            }
            if reader.move_to().expression(&[words::REDIRECT]).is_some() {
                let feed_func_token_id = close(reader);
                let feed_func_args_token_id = args_close(reader);
                return if let Some(Element::Function(mut dest)) =
                    Element::include(reader, &[ElTarget::Function])?
                {
                    dest.feeding(Self::new(
                        feed_func_token_id,
                        feed_func_args_token_id,
                        elements,
                        name,
                        tolerance,
                    )?);
                    dest.set_token(close(reader));
                    Ok(Some(dest))
                } else {
                    Err(E::NoDestFunction.linked(&reader.token()?.id))
                };
            }
            Ok(Some(Self::new(
                close(reader),
                args_close(reader),
                elements,
                name,
                tolerance,
            )?))
        } else {
            Ok(None)
        }
    }
}

impl Function {
    pub fn new(
        token: usize,
        args_token: usize,
        args: Vec<Element>,
        name: String,
        tolerance: bool,
    ) -> Result<Self, LinkedErr<E>> {
        Ok(Self {
            token,
            args_token,
            name,
            tolerance,
            feed: None,
            args,
        })
    }
    pub fn feeding(&mut self, func: Function) {
        if let Some(bound) = self.feed.as_mut() {
            bound.feeding(func);
        } else {
            self.feed = Some(Box::new(func));
        }
    }
    pub fn set_token(&mut self, token: usize) {
        self.token = token;
    }
    pub async fn get_processed_args<'a>(
        &self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        inputs: &'a [String],
        cx: &'a mut Context,
    ) -> Result<Vec<AnyValue>, operator::E> {
        let mut values: Vec<AnyValue> = vec![];
        for arg in self.args.iter() {
            values.push(
                arg.execute(owner, components, inputs, cx)
                    .await?
                    .ok_or(operator::E::NotAllArguamentsHasReturn)?,
            )
        }
        Ok(values)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn to_string(func: &Function) -> String {
            format!(
                "@{}{}{}{}{}",
                func.name,
                if func.tolerance { "?" } else { "" },
                if func.args.is_empty() { "" } else { "(" },
                func.args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join("; "),
                if func.args.is_empty() { "" } else { ")" }
            )
        }
        let mut nested: Vec<String> = vec![];
        let mut current = self;
        while let Some(feed) = current.feed.as_ref() {
            nested.push(to_string(feed));
            current = feed;
        }
        nested.reverse();
        write!(
            f,
            "{}{}{}",
            nested.join(" >> "),
            if nested.is_empty() { "" } else { " >> " },
            to_string(self)
        )
    }
}

impl Operator for Function {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        inputs: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let mut args: Vec<AnyValue> = self
                .get_processed_args(owner, components, inputs, cx)
                .await?;
            if let Some(func) = self.feed.as_ref() {
                args.insert(
                    0,
                    cx.get_fn(&func.name)
                        .ok_or(operator::E::NoFunctionExecutor(self.name.clone()))?(
                        func.get_processed_args(owner, components, inputs, cx)
                            .await?,
                        cx,
                    )
                    .await?,
                );
            };
            let executor = cx
                .get_fn(&self.name)
                .ok_or(operator::E::NoFunctionExecutor(self.name.clone()))?;
            Ok(Some(executor(args, cx).await?))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Function,
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests},
        reader::{chars, Reader, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let cx: Context = Context::unbound()?;
        let content = include_str!("../tests/reading/function.sibs").to_string();
        let len = content.split('\n').count();
        let mut reader = Reader::bound(content, &cx);
        let mut count = 0;
        while let Some(entity) = tests::report_if_err(&cx, Function::read(&mut reader))? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};")),
                "Line: {}",
                count + 1
            );
            count += 1;
        }
        assert_eq!(count, len);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let content = include_str!("../tests/reading/function.sibs").to_string();
        let len = content.split('\n').count();
        let mut reader = Reader::unbound(content);
        let mut count = 0;
        while let Some(entity) = Function::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(&format!("{entity}")),
                reader.get_fragment(&entity.token)?.content
            );
            for arg in entity.args.iter() {
                assert_eq!(
                    tests::trim_carets(&arg.to_string()),
                    reader.get_fragment(&arg.token())?.lined
                );
            }
            count += 1;
        }
        assert_eq!(count, len);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../tests/error/function.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            let func = Function::read(&mut reader);
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
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{chars, Reader, Reading},
    };

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader = Reader::bound(
            include_str!("../tests/processing/functions.sibs").to_string(),
            &cx,
        );
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(None, &[], &[], &mut cx)
                .await?
                .expect("Task returns some value");
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                result.get_as_string().expect("Task returns string value"),
                "true".to_owned()
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{ElTarget, Element, Function, Task},
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;

    impl Arbitrary for Function {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            if deep > MAX_DEEP {
                ("[a-z][a-z0-9]*".prop_map(String::from),)
                    .prop_map(|(name,)| Function {
                        args: vec![],
                        tolerance: false,
                        token: 0,
                        args_token: 0,
                        feed: None,
                        name: if name.is_empty() {
                            "min".to_owned()
                        } else {
                            name
                        },
                    })
                    .boxed()
            } else {
                (
                    "[a-z][a-z0-9]*".prop_map(String::from),
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
                        tolerance: false,
                        token: 0,
                        args_token: 0,
                        feed: None,
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

    fn reading(func: Function) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("test [\n{func};\n];");
            let mut reader = Reader::unbound(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
            }
            Ok(())
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
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
