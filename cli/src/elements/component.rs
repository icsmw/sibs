use crate::{
    elements::{ElTarget, Element, Gatekeeper, SimpleString, Task},
    error::LinkedErr,
    inf::{
        operator, scenario, Context, Execute, ExecutePinnedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, LocalVariablesMap, Scope, TokenGetter, TryExecute,
        Value, ValueRef, ValueTypeResult,
    },
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};
use std::{fmt, path::PathBuf};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Component {
    pub cwd: Option<PathBuf>,
    pub name: SimpleString,
    pub elements: Vec<Element>,
    pub token: usize,
    pub uuid: Uuid,
    pub variables: LocalVariablesMap,
}

impl Component {
    pub fn link(&mut self, variables: &mut GlobalVariablesMap) -> Result<(), operator::E> {
        self.variables = variables.withdraw(&self.uuid)?;
        Ok(())
    }
    pub fn get_name(&self) -> String {
        self.name.value.to_owned()
    }
    pub fn get_task(&self, name: &str) -> Option<(&Task, Vec<&Element>)> {
        let mut gatekeepers = Vec::new();
        for el in self.elements.iter() {
            if let Element::Task(task, _) = el {
                if task.get_name() == name {
                    return Some((task, gatekeepers));
                } else {
                    gatekeepers.clear();
                }
            } else if matches!(el, Element::Gatekeeper(..)) {
                gatekeepers.push(el);
            }
        }
        None
    }
    pub fn get_cwd(&self, cx: &Context) -> Result<Option<PathBuf>, scenario::E> {
        Ok(if let Some(path) = self.cwd.as_ref() {
            Some(cx.scenario.to_abs_path(path)?)
        } else {
            None
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
}

impl TryDissect<Component> for Component {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Component>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Component);
        let Some((before, _)) = reader.until().char(&[&chars::POUND_SIGN]) else {
            return Ok(None);
        };
        if !before.is_empty() {
            Err(E::UnrecognizedCode(before).by_reader(reader))?;
        }
        let _ = reader.move_to().char(&[&chars::POUND_SIGN]);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Err(E::NoComponentDefinition.by_reader(reader));
        }
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
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
        {
            Err(E::InvalidComponentName(name.clone()).by_reader(reader))?;
        }
        let (name, name_token) = (name, inner.token()?.id);
        let path = inner.rest().trim().to_string();
        let rest = if let Some((rest, _)) = reader.until().word(&[words::COMP]) {
            rest
        } else {
            reader.move_to().end()
        };
        if rest.trim().is_empty() {
            Err(E::NoComponentBody.linked(&name_token))?
        }
        let mut inner = reader.token()?.bound;
        let inner_token_id = reader.token()?.id;
        let mut elements: Vec<Element> = Vec::new();
        while let Some(el) = Element::include(&mut inner, &[ElTarget::Task, ElTarget::Gatekeeper])?
        {
            let _ = inner.move_to().char(&[&chars::SEMICOLON]);
            elements.push(el);
        }
        if elements.is_empty() {
            return Err(E::UnrecognizedCode(rest).linked(&inner_token_id));
        }
        Ok(Some(Component {
            uuid: Uuid::new_v4(),
            variables: LocalVariablesMap::default(),
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
    }
}

impl Dissect<Component, Component> for Component {}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#({}{}){}",
            self.name,
            self.cwd
                .as_ref()
                .map(|cwd| format!(": {}", cwd.display()))
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
                .map(|cwd| format!(": {}", cwd.display()))
                .unwrap_or_default(),
            self.elements
                .iter()
                .map(|el| format!("{};", el.format(&mut inner)))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl TokenGetter for Component {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for Component {
    fn linking<'a>(
        &'a self,
        variables: &mut GlobalVariablesMap,
        _: &'a Component,
        components: &'a [Component],
    ) -> Result<(), LinkedErr<operator::E>> {
        for el in self.elements.iter() {
            el.linking(variables, self, components)?;
        }
        Ok(())
    }
    fn expected<'a>(
        &'a self,
        _owner: &'a Component,
        _components: &'a [Component],
    ) -> ValueTypeResult {
        Ok(ValueRef::Empty)
    }
}

impl TryExecute for Component {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [Value],
        cx: Context,
        _sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let task = args
                .first()
                .and_then(|task| task.as_string())
                .ok_or_else(|| {
                    operator::E::NoTaskForComponent(self.name.to_string(), self.get_tasks_names())
                })?;
            let (task, gatekeepers) = self.get_task(&task).ok_or_else(|| {
                operator::E::TaskNotExists(
                    self.name.to_string(),
                    task.to_owned(),
                    self.get_tasks_names(),
                )
            })?;
            let sc = cx
                .scope
                .create(
                    format!("{}:{}", self.name, task.get_name()),
                    self.cwd.clone(),
                )
                .await?;
            let task_ref = task
                .as_reference(
                    owner,
                    components,
                    &args[1..],
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?;
            let skippable = Gatekeeper::skippable(
                gatekeepers,
                &task_ref,
                owner,
                components,
                cx.clone(),
                sc.clone(),
                token.clone(),
            )
            .await?;
            if skippable {
                cx.journal.debug(
                    task.get_name(),
                    format!("{task_ref} will be skipped because gatekeeper conclusion",),
                );
            }
            let result = if !skippable {
                task.execute(owner, components, &args[1..], cx, sc.clone(), token)
                    .await
            } else {
                Ok(None)
            };
            sc.destroy().await?;
            result
        })
    }
}

impl Execute for Component {}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Component, ElTarget, Element},
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let components = include_str!("../tests/reading/component.sibs")
            .split('\n')
            .collect::<Vec<&str>>();
        let tasks = include_str!("../tests/reading/tasks.sibs");
        read_string!(
            &Configuration::logs(false),
            &components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Component::dissect(reader))? {
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
            &Configuration::logs(false),
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
            .map(|v| format!("{v} task {{\nenv::is_os();\n}};"))
            .collect::<Vec<String>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let res = Component::dissect(reader);
                    println!("{res:?}");
                    assert!(res.is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::Component,
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, Journal, Scope, Value,
        },
        process_string,
        reader::{Dissect, Reader, Sources},
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
            &Configuration::logs(false),
            &include_str!("../tests/processing/component.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Component> = Vec::new();
                while let Some(task) = src.report_err_if(Component::dissect(reader))? {
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
                                .map(|s| Value::String(s.to_string()))
                                .collect::<Vec<Value>>(),
                            cx.clone(),
                            sc.clone(),
                            CancellationToken::new(),
                        )
                        .await?
                        .expect("component returns some value");
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
    use std::path::PathBuf;

    use crate::{
        elements::{Component, ElTarget, Element, SimpleString},
        inf::LocalVariablesMap,
    };
    use proptest::prelude::*;
    use uuid::Uuid;

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
                    uuid: Uuid::new_v4(),
                    variables: LocalVariablesMap::default(),
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
