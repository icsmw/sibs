use crate::{
    entry::{Block, Component, ElTarget, Element, SimpleString, VariableDeclaration},
    error::LinkedErr,
    inf::{
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
        term::{self, Term},
    },
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: SimpleString,
    pub declarations: Vec<VariableDeclaration>,
    pub dependencies: Vec<Element>,
    pub block: Option<Block>,
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
            let declarations: Vec<VariableDeclaration> = if stopped_on == chars::OPEN_SQ_BRACKET {
                vec![]
            } else if reader.until().char(&[&chars::CLOSE_BRACKET]).is_some() {
                reader.move_to().next();
                let mut declarations: Vec<VariableDeclaration> = vec![];
                let mut token = reader.token()?;
                while let Some(variable_declaration) = VariableDeclaration::read(&mut token.bound)?
                {
                    declarations.push(variable_declaration);
                }
                if !token.bound.rest().trim().is_empty() {
                    Err(
                        E::InvalidTaskArguments(token.bound.rest().trim().to_string())
                            .linked(&token.id),
                    )?
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
                    block: Some(block),
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
                .as_ref()
                .map(|b| b.to_string())
                .unwrap_or_default()
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
        if let Some(block) = self.block.as_ref() {
            block.display(term);
        }
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
            let block = self
                .block
                .as_ref()
                .ok_or_else(|| operator::E::NoTaskBlock(self.name.value.to_string()))?;
            if self.declarations.len() != args.len() {
                cx.gen_report(
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
            for (i, declaration) in self.declarations.iter().enumerate() {
                declaration.declare(args[i].to_owned(), cx).await?;
            }
            job.result(block.execute(owner, components, args, cx).await)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::Task,
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests::*},
        reader::{chars, Reader, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let cx: Context = Context::unbound()?;
        let mut reader =
            Reader::bound(include_str!("../tests/reading/tasks.sibs").to_string(), &cx);
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

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(include_str!("../tests/reading/tasks.sibs").to_string());
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
            if let Some(block) = entity.block.as_ref() {
                assert_eq!(
                    trim_carets(&block.to_string()),
                    trim_carets(&reader.get_fragment(&block.token)?.lined)
                );
            }
            for declaration in entity.declarations.iter() {
                assert_eq!(
                    trim_carets(&declaration.to_string()),
                    trim_carets(&reader.get_fragment(&declaration.token)?.lined)
                );
                assert_eq!(
                    trim_carets(&declaration.variable.to_string()),
                    trim_carets(&reader.get_fragment(&declaration.variable.token)?.lined)
                );
                assert_eq!(
                    trim_carets(&declaration.declaration.to_string()),
                    trim_carets(&reader.get_fragment(&declaration.declaration.token())?.lined)
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

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../tests/error/tasks.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
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
        entry::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{chars, Reader, Reading},
    };

    const VALUES: &[&[&str]] = &[&["a"], &["a", "b"], &["a"], &["a", "b"], &["a", "b", "c"]];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader = Reader::bound(
            include_str!("../tests/processing/tasks.sibs").to_string(),
            &cx,
        );
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
    use crate::entry::{Block, SimpleString, Task, VariableDeclaration};
    use proptest::prelude::*;

    impl Arbitrary for Task {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                prop::collection::vec(VariableDeclaration::arbitrary(), 0..=5),
                Block::arbitrary_with(deep),
                "[a-zA-Z_]*".prop_map(String::from),
            )
                .prop_map(|(declarations, block, name)| Task {
                    declarations,
                    block: Some(block),
                    token: 0,
                    dependencies: vec![],
                    name: SimpleString {
                        value: name,
                        token: 0,
                    },
                })
                .boxed()
        }
    }
}
