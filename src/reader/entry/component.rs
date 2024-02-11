use crate::{
    inf::{
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
        term::{self, Term},
    },
    reader::{
        chars,
        entry::{Function, Meta, Reading, Task},
        words, Reader, E,
    },
};
use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub struct Component {
    pub cwd: Option<PathBuf>,
    pub name: String,
    pub tasks: Vec<Task>,
    pub functions: Vec<Function>,
    pub meta: Option<Meta>,
    pub token: usize,
}

impl Component {
    pub fn get_task(&self, name: &str) -> Option<&Task> {
        self.tasks.iter().find(|t| t.name == name)
    }
}

impl Reading<Component> for Component {
    fn read(reader: &mut Reader) -> Result<Option<Component>, E> {
        if reader.move_to().char(&[&chars::POUND_SIGN]).is_some() {
            if reader
                .group()
                .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let name = inner
                    .until()
                    .char(&[&chars::COLON])
                    .map(|(v, _)| {
                        inner.move_to().next();
                        v
                    })
                    .unwrap_or_else(|| inner.move_to().end());
                if name.trim().is_empty() {
                    Err(E::EmptyComponentName)?;
                }
                if !Reader::is_ascii_alphabetic_and_alphanumeric(
                    &name,
                    &[&chars::UNDERSCORE, &chars::DASH],
                ) {
                    Err(E::InvalidComponentName)?;
                }
                let path = inner.rest().trim().to_string();
                let inner = if let Some((inner, _)) = reader.until().word(&[&words::COMP]) {
                    inner
                } else {
                    reader.move_to().end()
                };
                if inner.trim().is_empty() {
                    Err(E::NoComponentBody)?
                }
                let mut task_reader = reader.token()?.bound;
                let mut meta: Option<Meta> = None;
                if let Some(mt) = Meta::read(&mut task_reader)? {
                    meta = Some(mt);
                }
                let mut functions: Vec<Function> = vec![];
                while let Some(func) = Function::read(&mut task_reader)? {
                    functions.push(func);
                }
                let mut tasks: Vec<Task> = vec![];
                while let Some(task) = Task::read(&mut task_reader)? {
                    tasks.push(task);
                }
                Ok(Some(Component {
                    name,
                    functions,
                    cwd: if path.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(path))
                    },
                    tasks,
                    meta,
                    token: reader.token()?.id,
                }))
            } else {
                Err(E::NoGroup)?
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#[{}{}]{}{}\n{}",
            self.name,
            self.cwd
                .as_ref()
                .map(|cwd| format!(": {}", cwd.to_string_lossy()))
                .unwrap_or_default(),
            self.meta
                .as_ref()
                .map(|meta| meta.to_string())
                .unwrap_or_default(),
            self.functions
                .iter()
                .map(|function| format!("{function};"))
                .collect::<Vec<String>>()
                .join("\n"),
            self.tasks
                .iter()
                .map(|task| format!("{task};"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl term::Display for Component {
    fn display(&self, term: &mut Term) {
        term.bold("COMPONENT:\n");
        term.step_right();
        term.boldnl(&self.name);
        if let Some(meta) = self.meta.as_ref() {
            println!();
            meta.display(term);
        }
        term.step_left();
        term.bold("\nTASKS:\n");
        term.step_right();
        self.tasks.iter().filter(|t| t.has_meta()).for_each(|task| {
            task.display(term);
        });
        term.step_left();
    }
}

impl Operator for Component {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let task = args.first().ok_or_else(|| {
                cx.term.err(format!(
                    "No task provided for component \"{}\". Try to use \"sibs {} --help\".\n",
                    self.name, self.name
                ));
                operator::E::NoTaskForComponent(self.name.to_string())
            })?;
            let task = self.tasks.iter().find(|t| &t.name == task).ok_or_else(|| {
                cx.term.err(format!(
                    "Task \"{task}\" doesn't exist on component \"{}\". Try to use \"sibs {} --help\".\n",
                    self.name, self.name
                ));
                operator::E::TaskNotExists( task.to_owned(),self.name.to_string())
            })?;
            cx.term.with_title("COMPONENT", &self.name);
            cx.set_cwd(self.cwd.clone())?;
            task.process(owner, components, &args[1..], cx).await
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        inf::tests,
        reader::{
            entry::{Component, Reading},
            Reader, E,
        },
    };

    #[test]
    fn reading() -> Result<(), E> {
        let components = include_str!("../../tests/reading/component.sibs").to_string();
        let components = components.split('\n').collect::<Vec<&str>>();
        let tasks = include_str!("../../tests/reading/tasks.sibs");
        let mut reader = Reader::new(
            components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
        );
        let mut count = 0;
        while let Some(entity) = Component::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&format!("{entity}"))
            );
            count += 1;
        }
        assert_eq!(count, components.len());
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/component.sibs").to_string();
        let samples = samples
            .split('\n')
            .map(|v| format!("{v} [\n@os;\n];"))
            .collect::<Vec<String>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Component::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{
            entry::{Component, Reading},
            Reader,
        },
    };

    const VALUES: &[&[&str]] = &[
        &["test", "a"],
        &["test", "a", "b", "c"],
        &["test", "a", "b", "c"],
        &["test", "a", "b", "c"],
    ];

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::new(include_str!("../../tests/processing/component.sibs").to_string());
        let mut cursor: usize = 0;
        let mut components: Vec<Component> = vec![];
        while let Some(component) = Component::read(&mut reader)? {
            components.push(component);
        }
        for component in components.iter() {
            let result = component
                .process(
                    Some(component),
                    &components,
                    &VALUES[cursor]
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                    &mut cx,
                )
                .await?
                .expect("component returns some value");
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
    use std::path::PathBuf;

    use crate::{
        inf::tests::*,
        reader::entry::{component::Component, meta::Meta, task::Task},
    };
    use proptest::prelude::*;

    impl Arbitrary for Component {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            (
                "[a-zA-Z]*".prop_map(String::from),
                prop::collection::vec(Task::arbitrary_with(scope.clone()), 2..6),
                Meta::arbitrary_with(scope.clone()),
            )
                .prop_map(|(name, tasks, meta)| Component {
                    tasks,
                    meta: Some(meta),
                    name,
                    cwd: Some(PathBuf::new()),
                    functions: vec![],
                    token: 0,
                })
                .boxed()
        }
    }
}
