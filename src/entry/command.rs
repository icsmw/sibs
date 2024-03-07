use crate::{
    entry::{pattern_string_reader, Component, Element},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
        spawner,
        term::{self, Term},
        tracker::Logs,
    },
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Command {
    pub pattern: String,
    pub injections: Vec<(String, Element)>,
    pub token: usize,
}

impl Reading<Command> for Command {
    fn read(reader: &mut Reader) -> Result<Option<Command>, LinkedErr<E>> {
        if let Some((pattern, injections, token)) =
            pattern_string_reader::read(reader, chars::TILDA)?
        {
            Ok(Some(Command {
                pattern,
                injections,
                token,
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.pattern,)
    }
}

impl term::Display for Command {
    fn display(&self, term: &mut Term) {
        term.printnl(&self.pattern);
    }
}

impl Operator for Command {
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
            let mut output = self.pattern.clone();
            for (hook, injection) in self.injections.iter() {
                let val = injection
                    .execute(owner, components, inputs, cx)
                    .await?
                    .ok_or(operator::E::FailToExtractValue)?
                    .get_as_string()
                    .ok_or(operator::E::FailToGetValueAsString)?;
                let hook = format!("{{{}}}", hook);
                output = output.replace(&hook, &val);
            }
            let cwd = cx.cwd.as_ref().ok_or(operator::E::NoCurrentWorkingFolder)?;
            let job = cx
                .tracker
                .create_job(
                    &format!("{}: {}", cx.scenario.to_relative_path(cwd), output),
                    None,
                )
                .await?;
            match spawner::run(&output, cwd, &job).await {
                Ok(status) => {
                    if status.success() {
                        job.success();
                        Ok(Some(AnyValue::new(())))
                    } else {
                        job.fail();
                        Err(operator::E::SpawnedProcessExitWithError)
                    }
                }
                Err(e) => {
                    job.err(e.to_string());
                    job.fail();
                    Err(e)?
                }
            }
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::Command,
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{chars, Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(include_str!("../tests/reading/command.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Command::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&entity.to_string()),
            );
            count += 1;
        }
        assert_eq!(count, 16);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(include_str!("../tests/reading/command.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Command::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&entity.to_string()),
                reader.get_fragment(&entity.token)?.content
            );
            for (hook, el) in entity.injections.iter() {
                assert_eq!(*hook, reader.get_fragment(&el.token())?.content);
            }
            count += 1;
        }
        assert_eq!(count, 16);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        entry::command::{Command, Element},
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Command {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Command);
            let boxed = (
                prop::collection::vec(Element::arbitrary_with(scope.clone()), 1..=10),
                prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 10),
                prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 10),
            )
                .prop_map(|(injections, noise, hooks)| {
                    let mut pattern: String = String::new();
                    for (i, _el) in injections.iter().enumerate() {
                        pattern = format!("{}{{{}}}", noise[i], hooks[i].clone());
                    }
                    Command {
                        injections: injections
                            .iter()
                            .enumerate()
                            .map(|(i, el)| (hooks[i].clone(), el.clone()))
                            .collect::<Vec<(String, Element)>>(),
                        pattern,
                        token: 0,
                    }
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Command);
            boxed
        }
    }

    impl Command {
        pub fn arbitrary_primitive(scope: SharedScope) -> BoxedStrategy<Self> {
            scope.write().unwrap().include(Entity::Command);
            let boxed = "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|pattern| Command {
                    injections: vec![],
                    pattern,
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Command);
            boxed
        }
    }
}
