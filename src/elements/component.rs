use crate::{
    elements::{ElTarget, Element, Meta, SimpleString, Task},
    error::LinkedErr,
    inf::{
        operator, term, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Term,
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::{fmt, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Component {
    pub cwd: Option<PathBuf>,
    pub name: SimpleString,
    pub elements: Vec<Element>,
    pub token: usize,
}

impl Component {
    pub fn get_task(&self, name: &str) -> Option<&Task> {
        self.elements.iter().find_map(|el| {
            if let Element::Task(task, _) = el {
                if task.get_name() == name {
                    Some(task)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
    pub fn get_tasks(&self) -> Vec<&Task> {
        self.elements
            .iter()
            .filter_map(|el| {
                if let Element::Task(task, _) = el {
                    Some(task)
                } else {
                    None
                }
            })
            .collect::<Vec<&Task>>()
    }
    pub fn get_meta(&self) -> Vec<&Meta> {
        self.elements
            .iter()
            .filter_map(|el| {
                if let Element::Meta(meta, _) = el {
                    Some(meta)
                } else {
                    None
                }
            })
            .collect::<Vec<&Meta>>()
    }
    pub fn get_name(&self) -> &str {
        &self.name.value
    }
}

impl Reading<Component> for Component {
    fn read(reader: &mut Reader) -> Result<Option<Component>, LinkedErr<E>> {
        let close = reader.open_token();
        let restore = reader.pin();
        if let Some((before, _)) = reader.until().char(&[&chars::POUND_SIGN]) {
            if !before.is_empty() {
                Err(E::UnrecognizedCode(before).by_reader(reader))?;
            }
            let _ = reader.move_to().char(&[&chars::POUND_SIGN]);
            if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
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
                    Err(E::EmptyComponentName.by_reader(reader))?;
                }
                if !Reader::is_ascii_alphabetic_and_alphanumeric(
                    &name,
                    &[&chars::UNDERSCORE, &chars::DASH],
                ) {
                    Err(E::InvalidComponentName(name.clone()).by_reader(reader))?;
                }
                let (name, name_token) = (name, inner.token()?.id);
                let path = inner.rest().trim().to_string();
                let inner = if let Some((inner, _)) = reader.until().word(&[words::COMP]) {
                    inner
                } else {
                    reader.move_to().end()
                };
                if inner.trim().is_empty() {
                    Err(E::NoComponentBody.linked(&name_token))?
                }
                let mut inner = reader.token()?.bound;
                let mut elements: Vec<Element> = vec![];
                while let Some(el) = Element::include(
                    &mut inner,
                    &[ElTarget::Meta, ElTarget::Task, ElTarget::Function],
                )? {
                    let _ = inner.move_to().char(&[&chars::SEMICOLON]);
                    elements.push(el);
                }

                Ok(Some(Component {
                    name: SimpleString {
                        value: name,
                        token: name_token,
                    },
                    elements,
                    cwd: if path.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(path))
                    },
                    token: close(reader),
                }))
            } else {
                Err(E::NoComponentDefinition.by_reader(reader))?
            }
        } else {
            restore(reader);
            Ok(None)
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#({}{}){}",
            self.name,
            self.cwd
                .as_ref()
                .map(|cwd| format!(": {}", cwd.to_string_lossy()))
                .unwrap_or_default(),
            self.elements
                .iter()
                .map(|el| format!("{el};"))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl Formation for Component {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::Component)).right();
        format!(
            "#({}{})\n{}",
            self.name,
            self.cwd
                .as_ref()
                .map(|cwd| format!(": {}", cwd.to_string_lossy()))
                .unwrap_or_default(),
            self.elements
                .iter()
                .map(|el| format!("{};", el.format(&mut inner)))
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
        // if let Some(meta) = self.meta.as_ref() {
        //     println!();
        //     meta.display(term);
        // }
        term.step_left();
        term.bold("\nTASKS:\n");
        term.step_right();
        // self.tasks.iter().filter(|t| t.has_meta()).for_each(|task| {
        //     task.display(term);
        // });
        term.step_left();
    }
}

impl Operator for Component {
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
            let task = args.first().ok_or_else(|| {
                cx.term.err(format!(
                    "No task provided for component \"{}\". Try to use \"sibs {} --help\".\n",
                    self.name, self.name
                ));
                operator::E::NoTaskForComponent(self.name.to_string())
            })?;
            let task = self.get_task(task).ok_or_else(|| {
                cx.term.err(format!(
                    "Task \"{task}\" doesn't exist on component \"{}\". Try to use \"sibs {} --help\".\n",
                    self.name, self.name
                ));
                operator::E::TaskNotExists( task.to_owned(),self.name.to_string())
            })?;
            let job = cx.tracker.create_job(self.get_name(), None).await?;
            cx.set_cwd(self.cwd.clone())?;
            job.result(task.execute(owner, components, &args[1..], cx).await)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Component,
        error::LinkedErr,
        inf::{
            context::Context,
            operator::Operator,
            tests::{self, report_if_err},
        },
        reader::{Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let components = include_str!("../tests/reading/component.sibs").to_string();
        let components = components.split('\n').collect::<Vec<&str>>();
        let tasks = include_str!("../tests/reading/tasks.sibs");
        let mut reader = cx.reader().from_str(
            &components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
        );
        let mut count = 0;
        while let Some(entity) = report_if_err(&cx, Component::read(&mut reader))? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&entity.to_string()),
            );
            count += 1;
        }
        assert_eq!(count, components.len());
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let components = include_str!("../tests/reading/component.sibs").to_string();
        let components = components.split('\n').collect::<Vec<&str>>();
        let tasks = include_str!("../tests/reading/tasks.sibs");
        let mut reader = cx.reader().from_str(
            &components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
        );
        let mut count = 0;
        while let Some(entity) = Component::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&entity.to_string()),
                tests::trim_carets(&reader.get_fragment(&entity.token)?.lined)
            );
            assert_eq!(
                tests::trim_carets(&entity.name.to_string()),
                tests::trim_carets(&reader.get_fragment(&entity.name.token)?.lined)
            );
            for el in entity.elements.iter() {
                assert_eq!(
                    tests::trim_carets(&format!("{el}",)),
                    tests::trim_carets(&reader.get_fragment(&el.token())?.lined)
                );
            }
            count += 1;
        }
        assert_eq!(count, components.len());
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn error() -> Result<(), E> {
        let mut cx: Context = Context::create().unbound()?;
        let samples = include_str!("../tests/error/component.sibs");
        let samples = samples
            .split('\n')
            .map(|v| format!("{v} [\n@os;\n];"))
            .collect::<Vec<String>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = cx.reader().from_str(sample);
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
        elements::Component,
        inf::{
            operator::{Operator, E},
            Context,
        },
        reader::Reading,
    };

    const VALUES: &[&[&str]] = &[
        &["test", "a"],
        &["test", "a", "b", "c"],
        &["test", "a", "b", "c"],
        &["test", "a", "b", "c"],
    ];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/processing/component.sibs"));
        let mut cursor: usize = 0;
        let mut components: Vec<Component> = vec![];
        while let Some(component) = Component::read(&mut reader)? {
            components.push(component);
        }
        for component in components.iter() {
            let result = component
                .execute(
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

    use crate::elements::{Component, ElTarget, Element, SimpleString};
    use proptest::prelude::*;

    impl Arbitrary for Component {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                "[a-zA-Z]*".prop_map(String::from),
                prop::collection::vec(Element::arbitrary_with((vec![ElTarget::Task], deep)), 2..6),
                prop::collection::vec(Element::arbitrary_with((vec![ElTarget::Meta], deep)), 0..3),
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElTarget::Function], deep)),
                    0..3,
                ),
            )
                .prop_map(|(name, tasks, meta, funcs)| Component {
                    elements: [meta, funcs, tasks].concat(),
                    name: SimpleString {
                        value: name,
                        token: 0,
                    },
                    cwd: Some(PathBuf::new()),
                    token: 0,
                })
                .boxed()
        }
    }
}
