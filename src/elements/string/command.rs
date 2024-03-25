use crate::{
    elements::{string, Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, spawner, term, AnyValue, Context, Formation, FormationCursor, Logs, Operator,
        OperatorPinnedResult, Term,
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
        if let Some((pattern, injections, token)) = string::read(reader, chars::TILDA)? {
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

impl Formation for Command {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
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
        elements::Command,
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests},
        reader::{chars, Reader, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../../tests/reading/command.sibs"));
        let origins = include_str!("../../tests/reading/command.sibs")
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        while let Some(entity) = tests::report_if_err(&cx, Command::read(&mut reader))? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};")),
                "line {}",
                count + 1
            );
            assert_eq!(
                origins[count],
                tests::trim_carets(&format!("{entity};")),
                "line {}",
                count + 1
            );
            count += 1;
        }
        assert_eq!(count, 130);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../../tests/reading/command.sibs"));
        let mut count = 0;
        while let Some(entity) = Command::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(&entity.to_string()),
                reader.get_fragment(&entity.token)?.content
            );
            for (hook, el) in entity.injections.iter() {
                assert_eq!(*hook, reader.get_fragment(&el.token())?.content);
            }
            count += 1;
        }
        assert_eq!(count, 130);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Command, ElTarget, Element},
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Command {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            if deep > MAX_DEEP {
                "[a-z][a-z0-9]*"
                    .prop_map(String::from)
                    .prop_map(|pattern| Command {
                        injections: vec![],
                        pattern: if pattern.len() < 3 {
                            "min".to_owned()
                        } else {
                            pattern
                        },
                        token: 0,
                    })
                    .boxed()
            } else {
                (
                    prop::collection::vec(
                        Element::arbitrary_with((
                            vec![ElTarget::VariableName, ElTarget::Function, ElTarget::If],
                            deep,
                        )),
                        0..=2,
                    ),
                    prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 10),
                )
                    .prop_map(|(injections, noise)| {
                        let mut pattern: String = String::new();
                        for (i, el) in injections.iter().enumerate() {
                            pattern = format!(
                                "{}{{{el}}}",
                                if noise[i].len() < 3 { "min" } else { &noise[i] }
                            );
                        }
                        Command {
                            injections: injections
                                .iter()
                                .map(|el| (el.to_string(), el.clone()))
                                .collect::<Vec<(String, Element)>>(),
                            pattern,
                            token: 0,
                        }
                    })
                    .boxed()
            }
        }
    }
}
