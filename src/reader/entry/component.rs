use crate::reader::{
    chars,
    entry::{Meta, Reading, Task},
    words, Reader, E,
};
use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub struct Component {
    pub cwd: Option<PathBuf>,
    pub name: String,
    pub tasks: Vec<Task>,
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
                let mut tasks: Vec<Task> = vec![];
                while let Some(task) = Task::read(&mut task_reader)? {
                    tasks.push(task);
                }
                Ok(Some(Component {
                    name,
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
            "#[{}{}]{}\n{}",
            self.name,
            self.cwd
                .as_ref()
                .map(|cwd| format!(": {}", cwd.to_string_lossy()))
                .unwrap_or_default(),
            self.meta
                .as_ref()
                .map(|meta| meta.to_string())
                .unwrap_or_default(),
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
            // println!("{:?}: {}", comp.cwd, comp.name,);
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&format!("{entity}"))
            );
            count += 1;
        }
        assert_eq!(count, 5);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
