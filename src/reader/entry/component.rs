use crate::reader::{
    chars,
    entry::{Function, Meta, Reading, Task},
    words, Reader, E,
};
use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub struct Component {
    pub cwd: Option<PathBuf>,
    pub name: String,
    pub tasks: Vec<Task>,
    pub functions: Vec<Function>,
    pub meta: Option<Meta>,
    pub index: usize,
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
                    index: reader.token()?.id,
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
                .map(|task| task.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod test_component {
    use crate::reader::{
        entry::{Component, Reading},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let components = include_str!("./tests/normal/component.sibs").to_string();
        let components = components.split('\n').collect::<Vec<&str>>();
        let tasks = include_str!("./tests/normal/tasks.sibs");
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
        let samples = include_str!("./tests/error/component.sibs").to_string();
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
