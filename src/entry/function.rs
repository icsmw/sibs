use crate::{
    entry::{Component, ElTarget, Element, ElementExd, SimpleString},
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
    pub args: Vec<ElementExd>,
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
                ])
                .map(|(str, char)| (str, Some(char)))
                .unwrap_or_else(|| (reader.move_to().end(), None));
            if !Reader::is_ascii_alphabetic_and_alphanumeric(
                &name,
                &[&chars::UNDERSCORE, &chars::DASH, &chars::COLON],
            ) {
                Err(E::InvalidFunctionName.by_reader(reader))?;
            }
            let args_close = reader.open_token();
            if matches!(ends_with, Some(chars::SEMICOLON)) {
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
                if let Some(next) = reader.next().char() {
                    if !next.is_whitespace() {
                        Err(E::InvalidFunctionName.by_reader(reader))?;
                    }
                }
                true
            } else {
                false
            };
            //TODO: prevent BLOCK as argument (conflict with EACH)
            let mut elements: Vec<ElementExd> = vec![];
            loop {
                if reader.rest().trim().starts_with(words::DO_ON) {
                    return Ok(Some(Self::new(
                        close(reader),
                        args_close(reader),
                        elements,
                        name,
                        tolerance,
                    )?));
                }
                if let Some(el) = Element::include(
                    reader,
                    &[
                        ElTarget::Values,
                        ElTarget::Function,
                        ElTarget::If,
                        ElTarget::PatternString,
                        ElTarget::Reference,
                        ElTarget::VariableComparing,
                        ElTarget::VariableName,
                        ElTarget::Command,
                    ],
                )? {
                    elements.push(ElementExd::Element(el));
                    continue;
                }
                if let Some((content, stopped)) =
                    reader
                        .until()
                        .char(&[&chars::WS, &chars::SEMICOLON, &chars::REDIRECT])
                {
                    if !content.trim().is_empty() {
                        elements.push(ElementExd::SimpleString(SimpleString {
                            value: content.trim().to_string(),
                            token: reader.token()?.id,
                        }));
                        continue;
                    }
                    if stopped == chars::SEMICOLON
                        || (stopped == chars::REDIRECT && content.trim().ends_with(words::DO_ON))
                    {
                        return Ok(Some(Self::new(
                            close(reader),
                            args_close(reader),
                            elements,
                            name,
                            tolerance,
                        )?));
                    }
                    reader.move_to().next();
                    if stopped == chars::REDIRECT {
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
                            Ok(Some(dest))
                        } else {
                            Err(E::NoDestFunction.linked(&reader.token()?.id))
                        };
                    }
                } else {
                    let content = reader.move_to().end();
                    if !content.trim().is_empty() {
                        elements.push(ElementExd::SimpleString(SimpleString {
                            value: content.to_string(),
                            token: reader.token()?.id,
                        }));
                    }
                }
                if reader.done() {
                    break;
                }
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
        args: Vec<ElementExd>,
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
                "@{}{}{}{}",
                func.name,
                if func.tolerance { "?" } else { "" },
                if func.args.is_empty() { "" } else { " " },
                func.args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
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
            nested.join(" > "),
            if nested.is_empty() { "" } else { " > " },
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
        entry::Function,
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{chars, Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let content = include_str!("../tests/reading/function.sibs").to_string();
        let len = content.split('\n').count();
        let mut reader = Reader::unbound(content);
        let mut count = 0;
        while let Some(entity) = Function::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
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
            assert_eq!(
                tests::trim_carets(&format!("{entity};")),
                reader.get_fragment(&entity.token)?.content
            );
            // assert_eq!(
            //     tests::trim_carets(&args.to_string()),
            //     reader.get_fragment(&args.token)?.lined
            // );
            // for arg in args.args.iter() {
            //     assert_eq!(
            //         tests::trim_carets(&arg.to_string()),
            //         reader.get_fragment(&arg.token())?.lined
            //     );
            // }
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
            assert!(Function::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        entry::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{Reader, Reading},
    };

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../tests/processing/functions.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(None, &[], &[], &mut cx)
                .await?
                .expect("Task returns some value");
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
        entry::{element::ElementExd, function::Function, task::Task},
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Function {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Function);
            let boxed = (
                "[a-z][a-z0-9]*".prop_map(String::from),
                prop::collection::vec(ElementExd::arbitrary_with(scope.clone()), 0..=5),
            )
                .prop_map(|(name, args)| Function {
                    args,
                    tolerance: false,
                    token: 0,
                    args_token: 0,
                    feed: None,
                    name,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Function);
            boxed
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
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn test_run_task(
            args in any_with::<Function>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
