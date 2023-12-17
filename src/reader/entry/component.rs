use std::path::PathBuf;

use crate::reader::{
    chars,
    entry::{Group, Meta, Reading, Task},
    words, Reader, E,
};

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
        if reader.move_to_char(&[chars::POUND_SIGN])?.is_some() {
            let from = reader.pos;
            if let Some(group) = Group::read(reader)? {
                let mut inner = reader.inherit(group.inner);
                if let Some((name, _, _)) = inner.read_until(&[chars::COLON], true, true)? {
                    let path = inner.rest().trim().to_string();
                    let inner = if let Some((inner, _, _)) =
                        reader.read_until_word(&[words::COMP], &[], false)?
                    {
                        inner
                    } else {
                        let (_, inner) = reader.to_end();
                        inner
                    };
                    if inner.trim().is_empty() {
                        Err(E::NoComponentBody)?
                    }
                    let mut task_reader = reader.inherit(inner);
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
                        index: reader.get_index_until_current(from),
                    }))
                } else {
                    Err(E::NoColon)
                }
            } else {
                Err(E::NoGroup)?
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test_component {
    use crate::reader::{
        entry::{Component, Reading},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let components = include_str!("./tests/component.sibs").to_string();
        let components = components.split('\n').collect::<Vec<&str>>();
        let tasks = include_str!("./tests/tasks.sibs");
        let mut reader = Reader::new(
            components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
            &mut mapper,
            0,
        );
        while let Some(comp) = Component::read(&mut reader)? {
            println!("{comp:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
