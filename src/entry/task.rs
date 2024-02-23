use crate::{
    entry::{Block, Component, SimpleString, VariableDeclaration},
    inf::{
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
        term::{self, Term},
    },
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug)]
pub struct Task {
    pub name: SimpleString,
    pub declarations: Vec<VariableDeclaration>,
    pub block: Option<Block>,
    pub token: usize,
}

impl Task {
    pub fn has_meta(&self) -> bool {
        self.block
            .as_ref()
            .map(|b| b.meta.is_some())
            .unwrap_or(false)
    }
    pub fn get_name(&self) -> &str {
        &self.name.value
    }
}

impl Reading<Task> for Task {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
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
                Err(reader.report_err(&name_token, E::InvalidTaskName(name.clone()))?)?
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
                    Err(reader.report_err(
                        &token.id,
                        E::InvalidTaskArguments(token.bound.rest().trim().to_string()),
                    )?)?
                }
                declarations
            } else {
                Err(reader.report_err(&name_token, E::NoTaskArguments)?)?
            };
            if let Some(block) = Block::read(reader)? {
                if reader.move_to().char(&[&chars::SEMICOLON]).is_some() {
                    reader.move_to().next();
                }
                Ok(Some(Task {
                    name: SimpleString {
                        value: name,
                        token: name_token,
                    },
                    declarations,
                    token: close(reader),
                    block: Some(block),
                }))
            } else {
                Err(reader.report_err(&name_token, E::FailFindTaskActions)?)
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
            "{}{} {}",
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
            self.block
                .as_ref()
                .map(|b| format!("{b}"))
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
    fn process<'a>(
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
            job.result(block.process(owner, components, args, cx).await)
                .await
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::Task,
        inf::tests,
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::unbound(include_str!("../tests/reading/tasks.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Task::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 6);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), E> {
        let mut reader = Reader::unbound(include_str!("../tests/reading/tasks.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Task::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&format!("{entity};")),
                tests::trim_carets(&reader.get_fragment(&entity.token)?.lined)
            );
            assert_eq!(
                tests::trim_carets(&entity.name.value),
                tests::trim_carets(&reader.get_fragment(&entity.name.token)?.lined)
            );
            if let Some(block) = entity.block.as_ref() {
                assert_eq!(
                    tests::trim_carets(&block.to_string()),
                    tests::trim_carets(&reader.get_fragment(&block.token)?.lined)
                );
            }
            for declaration in entity.declarations.iter() {
                assert_eq!(
                    tests::trim_carets(&declaration.to_string()),
                    tests::trim_carets(&reader.get_fragment(&declaration.token)?.lined)
                );
                assert_eq!(
                    tests::trim_carets(&declaration.name.to_string()),
                    tests::trim_carets(&reader.get_fragment(&declaration.name.token)?.lined)
                );
                assert_eq!(
                    tests::trim_carets(&declaration.declaration.to_string()),
                    tests::trim_carets(
                        &reader.get_fragment(&declaration.declaration.token())?.lined
                    )
                );
            }
            count += 1;
        }
        assert_eq!(count, 6);
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
        reader::{Reader, Reading},
    };

    const VALUES: &[&[&str]] = &[&["a"], &["a", "b"], &["a"], &["a", "b"], &["a", "b", "c"]];

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../tests/processing/tasks.sibs").to_string());
        let mut cursor: usize = 0;
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .process(
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
        entry::{
            block::Block, simple_string::SimpleString, task::Task,
            variable_declaration::VariableDeclaration,
        },
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Task {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Task);
            let boxed = (
                prop::collection::vec(VariableDeclaration::arbitrary_with(scope.clone()), 0..=5),
                Block::arbitrary_with(scope.clone()),
                "[a-zA-Z_]*".prop_map(String::from),
            )
                .prop_map(|(declarations, block, name)| Task {
                    declarations,
                    block: Some(block),
                    token: 0,
                    name: SimpleString {
                        value: name,
                        token: 0,
                    },
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Task);
            boxed
        }
    }
}
