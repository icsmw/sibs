use crate::{
    elements::{Block, Component, ElTarget, Element, SimpleString},
    error::LinkedErr,
    inf::{operator, Context, Formation, FormationCursor, Operator, OperatorPinnedResult},
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: SimpleString,
    pub declarations: Vec<Element>,
    pub dependencies: Vec<Element>,
    pub block: Block,
    pub token: usize,
}

impl Task {
    pub fn get_name(&self) -> &str {
        &self.name.value
    }
}

impl Reading<Task> for Task {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token();
        if let Some((name, stopped_on)) = reader
            .until()
            .char(&[&chars::OPEN_BRACKET, &chars::OPEN_SQ_BRACKET])
        {
            let (name, name_token) = (name.trim().to_string(), reader.token()?.id);
            if stopped_on == chars::OPEN_BRACKET {
                reader.move_to().next();
            }
            if !Reader::is_ascii_alphabetic_and_alphanumeric(
                &name,
                &[&chars::UNDERSCORE, &chars::DASH],
            ) {
                Err(E::InvalidTaskName(name.clone()).linked(&name_token))?
            }
            let declarations: Vec<Element> = if stopped_on == chars::OPEN_SQ_BRACKET {
                vec![]
            } else if reader.until().char(&[&chars::CLOSE_BRACKET]).is_some() {
                reader.move_to().next();
                let mut declarations: Vec<Element> = vec![];
                let mut inner = reader.token()?.bound;
                while let Some(el) = Element::include(&mut inner, &[ElTarget::VariableDeclaration])?
                {
                    let _ = inner.move_to().char(&[&chars::SEMICOLON]);
                    declarations.push(el);
                }
                if !inner.is_empty() {
                    Err(E::InvalidTaskArguments(inner.rest().trim().to_string()).by_reader(&inner))?
                }
                declarations
            } else {
                Err(E::NoTaskArguments.linked(&name_token))?
            };
            let mut dependencies: Vec<Element> = vec![];
            if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                while let Some(el) = Element::include(&mut inner, &[ElTarget::Reference])? {
                    let _ = inner.move_to().char(&[&chars::SEMICOLON]);
                    dependencies.push(el);
                }
                if !inner.is_empty() {
                    Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
                }
            }
            if let Some(block) = Block::read(reader)? {
                Ok(Some(Task {
                    name: SimpleString {
                        value: name,
                        token: name_token,
                    },
                    declarations,
                    dependencies,
                    token: close(reader),
                    block,
                }))
            } else {
                Err(E::FailFindTaskActions.linked(&name_token))
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{} {}",
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
                        .join("; ")
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
        let mut inner = cursor.reown(Some(ElTarget::Task));
        format!(
            "{}{}{}{} {}",
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
                        .join("; ")
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

impl Operator for Task {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            if self.declarations.len() != args.len() {
                Err(operator::E::DismatchTaskArgumentsCount(
                    self.declarations.len(),
                    self.declarations
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    args.len(),
                    args.join(", "),
                )
                .by(self))?;
            }
            for (i, el) in self.declarations.iter().enumerate() {
                if let Element::VariableDeclaration(declaration, _) = el {
                    declaration
                        .execute(owner, components, &[args[i].to_owned()], cx)
                        .await?;
                } else {
                    return Err(operator::E::InvalidVariableDeclaration.by(self));
                }
            }
            self.block.execute(owner, components, args, cx).await
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{ElTarget, Element, Task},
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests::*},
        reader::{chars, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/reading/tasks.sibs"))?;
        let mut count = 0;
        while let Some(el) =
            report_if_err(&mut cx, Element::include(&mut reader, &[ElTarget::Task]))?
        {
            assert!(matches!(el, Element::Task(..)));
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(trim_carets(reader.recent()), trim_carets(&format!("{el};")));
            count += 1;
        }
        assert_eq!(count, 11);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/reading/tasks.sibs"))?;
        let mut count = 0;
        while let Some(el) = Element::include(&mut reader, &[ElTarget::Task])? {
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
                    trim_carets(&reader.get_fragment(&el.block.token)?.lined)
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
        assert_eq!(count, 11);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn error() -> Result<(), E> {
        let mut cx: Context = Context::create().unbound()?;
        let samples = include_str!("../tests/error/tasks.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = cx.reader().from_str(sample)?;
            assert!(Task::read(&mut reader).is_err());
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
        reader::{chars, Reading},
    };

    const VALUES: &[&[&str]] = &[&["a"], &["a", "b"], &["a"], &["a", "b"], &["a", "b", "c"]];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/processing/tasks.sibs"))?;
        let mut cursor: usize = 0;
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(
                    None,
                    &[],
                    &VALUES[cursor]
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                    &mut cx,
                )
                .await?
                .expect("Task returns some value");
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            cursor += 1;
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
        elements::{Block, ElTarget, Element, SimpleString, Task},
        inf::{operator::E, tests::*, Context},
        reader::Reading,
    };
    use proptest::prelude::*;

    impl Arbitrary for Task {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElTarget::VariableDeclaration], deep)),
                    0..=5,
                ),
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElTarget::Reference], deep)),
                    0..=5,
                ),
                Block::arbitrary_with(deep),
                "[a-zA-Z_]*".prop_map(String::from),
            )
                .prop_map(|(declarations, dependencies, block, name)| Task {
                    declarations,
                    block,
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

    fn reading(task: Task) -> Result<(), E> {
        get_rt().block_on(async {
            let mut cx = Context::create().unbound()?;
            let origin = format!("{task};");
            let mut reader = cx.reader().from_str(&origin)?;
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
            args in any_with::<Task>(0)
        ) {
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
