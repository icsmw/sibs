use crate::{
    elements::{
        Boolean, Element, ElementRef, Integer, Metadata, PatternString, Reference, SimpleString,
        TokenGetter,
    },
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult,
        ExpectedValueType, Formation, FormationCursor, LinkingResult, PrevValueExpectation,
        Processing, TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Task {
    // TODO: replace SimpleString with Element
    pub name: SimpleString,
    pub declarations: Vec<Element>,
    pub dependencies: Vec<Element>,
    pub block: Box<Element>,
    pub token: usize,
}

impl Task {
    pub fn get_name(&self) -> &str {
        &self.name.value
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn get_args_values<'a>(
        &'a self,
        cx: ExecuteContext<'a>,
    ) -> Result<Vec<Value>, LinkedErr<operator::E>> {
        if self.declarations.len() != cx.args.len() {
            Err(operator::E::DismatchTaskArgumentsCount(
                self.declarations.len(),
                self.declarations
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                cx.args.len(),
                cx.args
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .by(self))?;
        }
        let mut values = Vec::new();
        for (i, el) in self.declarations.iter().enumerate() {
            if let Element::VariableDeclaration(declaration, _) = el {
                values.push(
                    declaration
                        .get_val(cx.clone().args(&[cx.args[i].to_owned()]))
                        .await?,
                );
            } else {
                return Err(operator::E::InvalidVariableDeclaration.by(self));
            }
        }
        Ok(values)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn as_reference<'a>(
        &'a self,
        cx: ExecuteContext<'a>,
    ) -> Result<Reference, LinkedErr<operator::E>> {
        let mut inputs = Vec::new();
        for arg in self.get_args_values(cx).await?.into_iter() {
            if let Some(v) = arg.as_num() {
                inputs.push(Element::Integer(
                    Integer { value: v, token: 0 },
                    Metadata::empty(),
                ));
            } else if let Value::bool(v) = arg {
                inputs.push(Element::Boolean(
                    Boolean { value: v, token: 0 },
                    Metadata::empty(),
                ));
            } else if let Some(value) = arg.as_string() {
                inputs.push(Element::PatternString(
                    PatternString {
                        elements: vec![Element::SimpleString(
                            SimpleString { value, token: 0 },
                            Metadata::empty(),
                        )],
                        token: 0,
                    },
                    Metadata::empty(),
                ));
            } else {
                return Err(operator::E::NoneStringTaskArgumentForReference.by(self));
            }
        }
        Ok(Reference {
            path: vec![self.get_name().to_owned()],
            inputs,
            token: 0,
        })
    }
}

impl TryDissect<Task> for Task {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Task);
        let Some(_) = reader.move_to().char(&[&chars::AT]) else {
            return Ok(None);
        };
        let Some((name, stopped_on)) = reader
            .until()
            .char(&[&chars::OPEN_BRACKET, &chars::OPEN_CURLY_BRACE])
        else {
            return Ok(None);
        };
        let (name, name_token) = (name.trim().to_string(), reader.token()?.id);
        if stopped_on == chars::OPEN_BRACKET {
            reader.move_to().next();
        }
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
        {
            Err(E::InvalidTaskName(name.clone()).linked(&name_token))?
        }
        let declarations: Vec<Element> = if stopped_on == chars::OPEN_CURLY_BRACE {
            Vec::new()
        } else if reader.until().char(&[&chars::CLOSE_BRACKET]).is_some() {
            reader.move_to().next();
            let mut declarations: Vec<Element> = Vec::new();
            let mut inner = reader.token()?.bound;
            while let Some(el) = Element::include(&mut inner, &[ElementRef::VariableDeclaration])?
            {
                let _ = inner.move_to().char(&[&chars::COMMA]);
                declarations.push(el);
            }
            if !inner.is_empty() {
                Err(E::InvalidTaskArguments(inner.rest().trim().to_string()).by_reader(&inner))?
            }
            declarations
        } else {
            Err(E::NoTaskArguments.linked(&name_token))?
        };
        let mut dependencies: Vec<Element> = Vec::new();
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            while let Some(el) = Element::include(
                &mut inner,
                &[ElementRef::Reference, ElementRef::VariableAssignation],
            )? {
                let _ = inner.move_to().char(&[&chars::SEMICOLON]);
                dependencies.push(el);
            }
            if !inner.is_empty() {
                Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
            }
        }
        if let Some(block) = Element::include(reader, &[ElementRef::Block])? {
            Ok(Some(Task {
                name: SimpleString {
                    value: name,
                    token: name_token,
                },
                declarations,
                dependencies,
                token: close(reader),
                block: Box::new(block),
            }))
        } else {
            Err(E::FailFindTaskActions.linked(&name_token))
        }
    }
}

impl Dissect<Task, Task> for Task {}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "@{}{}{} {}",
            self.name.value,
            if self.declarations.is_empty() && self.dependencies.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.declarations
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            },
            if self.dependencies.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.dependencies
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join(";")
                )
            },
            self.block
        )
    }
}

impl Formation for Task {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Task));
        format!(
            "@{}{}{}{} {}",
            cursor.offset_as_string(),
            self.name.value,
            if self.declarations.is_empty() && self.dependencies.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.declarations
                        .iter()
                        .map(|d| d.format(&mut inner))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            },
            if self.dependencies.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.dependencies
                        .iter()
                        .map(|d| d.format(&mut inner))
                        .collect::<Vec<String>>()
                        .join(";")
                )
            },
            self.block.format(&mut inner)
        )
    }
}

impl TokenGetter for Task {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Task {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            for el in self.dependencies.iter() {
                el.verification(owner, components, prev, cx).await?;
            }
            for el in self.declarations.iter() {
                el.verification(owner, components, prev, cx).await?;
            }
            self.block.verification(owner, components, prev, cx).await
        })
    }
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            for el in self.dependencies.iter() {
                el.linking(owner, components, prev, cx).await?;
            }
            for el in self.declarations.iter() {
                el.linking(owner, components, prev, cx).await?;
            }
            self.block.linking(owner, components, prev, cx).await
        })
    }

    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            let mut args: Vec<ValueRef> = Vec::new();
            for el in self.declarations.iter() {
                args.push(el.expected(owner, components, prev, cx).await?);
            }
            Ok(ValueRef::Task(
                args,
                Box::new(self.block.expected(owner, components, prev, cx).await?),
            ))
        })
    }
}

impl Processing for Task {}

impl TryExecute for Task {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            if self.declarations.len() != cx.args.len() {
                Err(operator::E::DismatchTaskArgumentsCount(
                    self.declarations.len(),
                    self.declarations
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    cx.args.len(),
                    cx.args
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
                .by(self))?;
            }
            for (i, el) in self.declarations.iter().enumerate() {
                el.execute(cx.clone().args(&[cx.args[i].to_owned()]))
                    .await?;
            }
            for dependency in self.dependencies.iter() {
                dependency.execute(cx.clone().args(&[])).await?;
            }
            let result = self.block.execute(cx.clone()).await?;
            Ok(cx.sc.get_retreat().await?.unwrap_or(result))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Element, ElementRef, Task, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/tasks.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    assert!(matches!(el, Element::Task(..)));
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(trim_carets(reader.recent()), trim_carets(&format!("{el};")));
                    count += 1;
                }
                assert!(reader.rest().trim().is_empty());
                assert_eq!(count, 11);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn deps() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/deps.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    assert!(matches!(el, Element::Task(..)));
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(trim_carets(reader.recent()), trim_carets(&format!("{el};")));
                    count += 1;
                }
                assert!(reader.rest().trim().is_empty());
                assert_eq!(count, 1);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/tasks.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert!(matches!(el, Element::Task(..)));
                    if let Element::Task(el, _) = el {
                        assert_eq!(
                            trim_carets(&format!("{el}")),
                            trim_carets(&reader.get_fragment(&el.token)?.lined)
                        );
                        assert_eq!(
                            trim_carets(&el.name.value),
                            trim_carets(&reader.get_fragment(&el.name.token)?.lined)
                        );
                        assert_eq!(
                            trim_carets(&el.block.to_string()),
                            trim_carets(&reader.get_fragment(&el.block.token())?.lined)
                        );
                        for declaration in el.declarations.iter() {
                            assert_eq!(
                                trim_carets(&declaration.to_string()),
                                trim_carets(&reader.get_fragment(&declaration.token())?.lined)
                            );
                        }
                        for dependency in el.dependencies.iter() {
                            assert_eq!(
                                trim_carets(&dependency.to_string()),
                                trim_carets(&reader.get_fragment(&dependency.token())?.lined)
                            );
                        }
                    }
                    count += 1;
                }
                assert!(reader.rest().trim().is_empty());
                assert_eq!(count, 11);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../tests/error/tasks.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(Task::dissect(reader).is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::{Element, ElementRef},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope, Value,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    const VALUES: &[&[&str]] = &[&["a"], &["a", "b"], &["a"], &["a", "b"], &["a", "b", "c"]];

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/tasks.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for (i, task) in tasks.iter().enumerate() {
                    let result = task
                        .execute(
                            ExecuteContext::unbound(cx.clone(), sc.clone()).args(
                                &VALUES[i]
                                    .iter()
                                    .map(|s| Value::String(s.to_string()))
                                    .collect::<Vec<Value>>(),
                            ),
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

    #[tokio::test]
    async fn deps() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/deps.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Component]))?
                {
                    components.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(components)
            },
            |components: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for component in components.iter() {
                    if !component.as_component()?.name.value.ends_with("_run") {
                        continue;
                    }
                    let result = component
                        .execute(
                            ExecuteContext::unbound(cx.clone(), sc.clone())
                                .owner(Some(component))
                                .components(&components)
                                .args(&[
                                    Value::String("test".to_owned()),
                                    Value::String("a".to_owned()),
                                    Value::String("b".to_owned()),
                                ]),
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
        elements::{Element, ElementRef, SimpleString, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Task {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElementRef::VariableDeclaration], deep)),
                    0..=5,
                ),
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElementRef::Reference], deep)),
                    0..=5,
                ),
                Element::arbitrary_with((vec![ElementRef::Block], deep)),
                "[a-zA-Z_]*".prop_map(String::from),
            )
                .prop_map(|(declarations, dependencies, block, name)| Task {
                    declarations,
                    block: Box::new(block),
                    token: 0,
                    dependencies,
                    name: SimpleString {
                        value: name,
                        token: 0,
                    },
                })
                .boxed()
        }
    }

    fn reading(task: Task) {
        get_rt().block_on(async {
            let origin = format!("{task};");
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
            args in any_with::<Task>(0)
        ) {
            reading(args.clone());
        }
    }
}
