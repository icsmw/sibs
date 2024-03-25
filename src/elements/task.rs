use crate::{
    elements::{Block, Component, ElTarget, Element, SimpleString},
    error::LinkedErr,
    inf::{
        operator, term, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Term,
    },
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
    pub fn has_meta(&self) -> bool {
        true
        // self.block
        //     .as_ref()
        //     .map(|b| b.meta.is_some())
        //     .unwrap_or(false)
    }
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
                    Err(E::UnrecognizedCode(inner.rest().to_string()).by_reader(&inner))?;
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
            if self.declarations.is_empty() {
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
            if self.declarations.is_empty() {
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
impl term::Display for Task {
    fn display(&self, term: &mut Term) {
        term.bold(format!("{}[{}]", term.offset(), self.name.value));
        println!();
        term.step_right();
        term.print(format!(
            "{}USAGE: {}{}{}",
            term.offset(),
            self.name.value,
            if self.declarations.is_empty() {
                ""
            } else {
                " "
            },
            self.declarations
                .iter()
                .map(term::Display::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        ));
        println!();
        self.block.display(term);
        term.step_left();
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
            let job = cx.tracker.create_job(self.get_name(), None).await?;
            if self.declarations.len() != args.len() {
                cx.sources.gen_report(
                    &self.name.token,
                    format!(
                        "Declared {} argument(s) ([{}]); passed {} argument(s) ([{}])",
                        self.declarations.len(),
                        self.declarations
                            .iter()
                            .map(|d| d.to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                        args.len(),
                        args.join(", ")
                    ),
                )?;
                Err(operator::E::DismatchTaskArgumentsCount)?;
            }
            for (i, el) in self.declarations.iter().enumerate() {
                if let Element::VariableDeclaration(declaration, _) = el {
                    declaration.declare(args[i].to_owned(), cx).await?;
                } else {
                    return Err(operator::E::InvalidVariableDeclaration);
                }
            }
            job.result(self.block.execute(owner, components, args, cx).await)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Task,
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests::*},
        reader::{chars, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/reading/tasks.sibs"));
        let mut count = 0;
        while let Some(entity) = report_if_err(&cx, Task::read(&mut reader))? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                trim_carets(reader.recent()),
                trim_carets(&format!("{entity};"))
            );
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
            .from_str(include_str!("../tests/reading/tasks.sibs"));
        let mut count = 0;
        while let Some(entity) = Task::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                trim_carets(&format!("{entity}")),
                trim_carets(&reader.get_fragment(&entity.token)?.lined)
            );
            assert_eq!(
                trim_carets(&entity.name.value),
                trim_carets(&reader.get_fragment(&entity.name.token)?.lined)
            );
            assert_eq!(
                trim_carets(&entity.block.to_string()),
                trim_carets(&reader.get_fragment(&entity.block.token)?.lined)
            );
            for declaration in entity.declarations.iter() {
                assert_eq!(
                    trim_carets(&declaration.to_string()),
                    trim_carets(&reader.get_fragment(&declaration.token())?.lined)
                );
            }
            for dependency in entity.dependencies.iter() {
                assert_eq!(
                    trim_carets(&dependency.to_string()),
                    trim_carets(&reader.get_fragment(&dependency.token())?.lined)
                );
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
            let mut reader = cx.reader().from_str(sample);
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
        reader::{chars, Reader, Reading},
    };

    const VALUES: &[&[&str]] = &[&["a"], &["a", "b"], &["a"], &["a", "b"], &["a", "b", "c"]];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/processing/tasks.sibs"));
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
    use crate::elements::{Block, ElTarget, Element, SimpleString, Task};
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
}
