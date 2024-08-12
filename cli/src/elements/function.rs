use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
        Scope,
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FuncArg {
    pub value: AnyValue,
    pub token: usize,
}

impl FuncArg {
    pub fn new(value: AnyValue, token: usize) -> Self {
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
    pub feed: Option<Box<Function>>,
    pub token: usize,
    pub args_token: usize,
}

impl Reading<Function> for Function {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        reader.move_to().any();
        let restore = reader.pin();
        let close = reader.open_token(ElTarget::Function);
        let Some((name, mut stop)) = reader.until().char(&[
            &chars::OPEN_BRACKET,
            &chars::CARET,
            &chars::SEMICOLON,
            &chars::WS,
            &chars::OPEN_BRACKET,
            &chars::CLOSE_BRACKET,
        ]) else {
            restore(reader);
            return Ok(None);
        };
        if stop == chars::WS {
            if reader.move_to().char(&[&chars::OPEN_BRACKET]).is_none() {
                restore(reader);
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
            restore(reader);
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
                if inner.move_to().char(&[&chars::SEMICOLON]).is_none() && !inner.is_empty() {
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
        if reader.move_to().expression(&[words::REDIRECT]).is_some() {
            let feed_func_token_id = close(reader);
            let feed_func_args_token_id = args_close(reader);
            return if let Some(Element::Function(mut dest, _)) =
                Element::include(reader, &[ElTarget::Function])?
            {
                dest.feeding(Self::new(
                    feed_func_token_id,
                    feed_func_args_token_id,
                    elements,
                    name,
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
        )?))
    }
}

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
    pub fn get_feeding(&self) -> Vec<&Function> {
        let mut feeding: Vec<&Function> = Vec::new();
        let mut current = self;
        while let Some(feed) = current.feed.as_ref() {
            feeding.push(feed.as_ref());
            current = feed;
        }
        feeding.reverse();
        feeding
    }
    pub async fn get_processed_args<'a>(
        &self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        inputs: &'a [String],
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
                    inputs,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?
                .ok_or(operator::E::NotAllArguamentsHasReturn)?,
                arg.token(),
            ))
        }
        Ok(values)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn to_string(func: &Function) -> String {
            format!(
                "{}({})",
                func.name,
                func.args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join("; "),
            )
        }
        let feeding: Vec<String> = self.get_feeding().iter().map(|f| to_string(f)).collect();
        write!(
            f,
            "{}{}{}",
            feeding.join(" >> "),
            if feeding.is_empty() { "" } else { " >> " },
            to_string(self)
        )
    }
}

impl Formation for Function {
    fn elements_count(&self) -> usize {
        self.args.len() + self.get_feeding().len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        fn formated(func: &Function, cursor: &mut FormationCursor) -> String {
            format!(
                "{}({}{}",
                func.name,
                func.args
                    .iter()
                    .map(|arg| format!(
                        "\n{}{}",
                        cursor.right().offset_as_string(),
                        arg.format(&mut cursor.reown(Some(ElTarget::Function)).right())
                    ))
                    .collect::<Vec<String>>()
                    .join("; "),
                if func.args.is_empty() {
                    ")".to_string()
                } else {
                    format!("\n{})", cursor.offset_as_string_if(&[ElTarget::Block]))
                }
            )
        }
        let feeding = self.get_feeding();
        let output = if self.to_string().len() > cursor.max_len()
            || self.args.len() > cursor.max_args()
            || feeding.len() > cursor.max_args()
        {
            format!(
                "{}{}{}{}",
                cursor.offset_as_string_if(&[ElTarget::Block]),
                feeding
                    .iter()
                    .map(|f| formated(f, cursor))
                    .collect::<Vec<String>>()
                    .join(" >> "),
                if feeding.is_empty() { "" } else { " >> " },
                formated(self, cursor)
            )
        } else {
            format!(
                "{}{}",
                cursor.offset_as_string_if(&[ElTarget::Block, ElTarget::Component]),
                self
            )
        };
        format!(
            "{output}{}",
            if cursor.parent.is_none() { ";\n" } else { "" }
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
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let mut args: Vec<FuncArg> = self
                .get_processed_args(
                    owner,
                    components,
                    inputs,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?;
            if let Some(func) = self.feed.as_ref() {
                args.insert(
                    0,
                    FuncArg::new(
                        cx.execute(
                            &func.name,
                            func.get_processed_args(
                                owner,
                                components,
                                inputs,
                                cx.clone(),
                                sc.clone(),
                                token,
                            )
                            .await?,
                            func.args_token,
                            sc.clone(),
                        )
                        .await?,
                        func.token(),
                    ),
                );
            };
            Ok(Some(
                cx.execute(&self.name, args, self.args_token, sc).await?,
            ))
        })
    }
}

#[cfg(test)]
mod reading {

    use crate::{
        elements::Function,
        error::LinkedErr,
        inf::{operator::Operator, tests::*, Configuration},
        read_string,
        reader::{chars, Reader, Reading, Sources, E},
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
                while let Some(entity) = src.report_err_if(Function::read(reader))? {
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
                while let Some(entity) = src.report_err_if(Function::read(reader))? {
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
                let func = Function::read(reader);
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
        elements::Task,
        error::LinkedErr,
        inf::{
            operator::{Operator, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Reading, Sources},
    };

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/functions.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Task> = Vec::new();
                while let Some(task) = src.report_err_if(Task::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Task>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Task>, cx: Context, sc: Scope, _: Journal| async move {
                for task in tasks.iter() {
                    let result = task
                        .execute(
                            None,
                            &[],
                            &[],
                            cx.clone(),
                            sc.clone(),
                            CancellationToken::new(),
                        )
                        .await?
                        .expect("Task returns some value");
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
        reader::{Reader, Reading, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Function {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            if deep > MAX_DEEP {
                ("[a-z][a-z0-9_]*".prop_map(String::from),)
                    .prop_map(|(name,)| Function {
                        args: Vec::new(),
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
                    "[a-z][a-z0-9_]*".prop_map(String::from),
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

    fn reading(func: Function) {
        get_rt().block_on(async {
            let origin = format!("test [\n{func};\n];");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    while let Some(task) = src.report_err_if(Task::read(reader))? {
                        assert_eq!(format!("{task};"), origin);
                    }
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
