use crate::{
    elements::{ElTarget, Element, SimpleString, Task},
    error::LinkedErr,
    inf::{operator, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Scope},
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
    pub fn get_tasks_names(&self) -> Vec<String> {
        self.get_tasks()
            .iter()
            .map(|el| el.get_name().to_owned())
            .collect::<Vec<String>>()
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
                let mut elements: Vec<Element> = Vec::new();
                while let Some(el) =
                    Element::include(&mut inner, &[ElTarget::Task, ElTarget::Function])?
                {
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

impl Operator for Component {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: Context,
        _sc: Scope,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let task = args.first().ok_or_else(|| {
                operator::E::NoTaskForComponent(self.name.to_string(), self.get_tasks_names())
            })?;
            let task = self.get_task(task).ok_or_else(|| {
                operator::E::TaskNotExists(
                    self.name.to_string(),
                    task.to_owned(),
                    self.get_tasks_names(),
                )
            })?;
            let sc = Scope::init(if let Some(path) = self.cwd.as_ref() {
                Some(cx.scenario.to_abs_path(path)?)
            } else {
                None
            });
            let result = task
                .execute(owner, components, &args[1..], cx, sc.clone())
                .await;
            sc.destroy().await?;
            result
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Component, ElTarget, Element},
        error::LinkedErr,
        inf::{operator::Operator, tests::*, Configuration},
        read_string,
        reader::{Reader, Reading, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let components = include_str!("../tests/reading/component.sibs")
            .split('\n')
            .collect::<Vec<&str>>();
        let tasks = include_str!("../tests/reading/tasks.sibs");
        read_string!(
            &Configuration::logs(),
            &components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Component::read(reader))? {
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&entity.to_string()),
                    );
                    count += 1;
                }
                assert_eq!(count, components.len());
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        let components = include_str!("../tests/reading/component.sibs")
            .split('\n')
            .collect::<Vec<&str>>();
        let tasks = include_str!("../tests/reading/tasks.sibs");
        read_string!(
            &Configuration::logs(),
            &components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Component]))?
                {
                    assert!(matches!(el, Element::Component(..)));
                    assert_eq!(
                        trim_carets(&el.to_string()),
                        trim_carets(&reader.get_fragment(&el.token())?.lined)
                    );
                    if let Element::Component(el, _) = el {
                        assert_eq!(
                            trim_carets(&el.name.to_string()),
                            trim_carets(&reader.get_fragment(&el.name.token)?.lined)
                        );
                        for el in el.elements.iter() {
                            if let Element::Task(el, _) = el {
                                assert_eq!(
                                    trim_carets(&format!("{el}",)),
                                    trim_carets(&reader.get_fragment(&el.token())?.lined)
                                );
                            } else {
                                assert_eq!(
                                    trim_carets(&format!("{el}",)),
                                    trim_carets(&reader.get_fragment(&el.token())?.lined)
                                );
                            }
                        }
                    }
                    count += 1;
                }
                assert_eq!(count, components.len());
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../tests/error/component.sibs");
        let samples = samples
            .split('\n')
            .map(|v| format!("{v} [\n@os;\n];"))
            .collect::<Vec<String>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(Component::read(reader).is_err());
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
        elements::Component,
        error::LinkedErr,
        inf::{
            operator::{Operator, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{Reader, Reading, Sources},
    };

    const VALUES: &[&[&str]] = &[
        &["test", "a"],
        &["test", "a", "b", "c"],
        &["test", "a", "b", "c"],
        &["test", "a", "b", "c"],
    ];

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(),
            &include_str!("../tests/processing/component.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Component> = Vec::new();
                while let Some(task) = src.report_err_if(Component::read(reader))? {
                    components.push(task);
                }
                Ok::<Vec<Component>, LinkedErr<E>>(components)
            },
            |components: Vec<Component>, cx: Context, sc: Scope, _: Journal| async move {
                for (i, component) in components.iter().enumerate() {
                    let result = component
                        .execute(
                            Some(component),
                            &components,
                            &VALUES[i]
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>(),
                            cx.clone(),
                            sc.clone(),
                        )
                        .await?
                        .expect("component returns some value");
                    assert_eq!(
                        result.get_as_string().expect("Task returns string value"),
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
