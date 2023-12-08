use uuid::Uuid;

use crate::parser::{
    chars,
    entry::{Reading, VariableDeclaration},
    reader, Reader, E,
};

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub declarations: Vec<VariableDeclaration>,
    pub uuid: Uuid,
}

impl Reading<Task> for Task {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if let Some((name, stopped_on, uuid)) =
            reader.read_until(&[chars::OPEN_BRACKET, chars::OPEN_SQ_BRACKET], true, false)?
        {
            let declarations: Vec<VariableDeclaration> = if stopped_on == chars::OPEN_SQ_BRACKET {
                vec![]
            } else if let Some((content, _stopped_on, _uuid)) =
                reader.read_until(&[chars::CLOSE_BRACKET], true, false)?
            {
                let mut declarations: Vec<VariableDeclaration> = vec![];
                let mut vars_reader = reader.inherit(content);
                while let Some(variable_declaration) = VariableDeclaration::read(&mut vars_reader)?
                {
                    declarations.push(variable_declaration);
                }
                declarations
            } else {
                Err(E::NoTaskArguments)?
            };
            if stopped_on == chars::OPEN_BRACKET && !reader.move_to_char(chars::OPEN_SQ_BRACKET)? {
                Err(E::NoTaskActions)?
            }
            let actions = if let Some((content, stopped_on, uuid)) =
                reader.read_until(&[chars::CLOSE_SQ_BRACKET], true, false)?
            {
                Some((content, uuid))
            } else {
                Err(E::FailFindTaskActions)?
            };
            Ok(Some(Task::new(
                uuid,
                reader,
                name.trim().to_string(),
                declarations,
            )?))
        } else {
            Ok(None)
        }
    }
}

impl Task {
    pub fn new(
        uuid: Uuid,
        parent: &mut Reader,
        name: String,
        declarations: Vec<VariableDeclaration>,
    ) -> Result<Self, E> {
        Ok(Self {
            uuid,
            name,
            declarations: vec![],
        })
    }
}

// #[cfg(test)]
// mod test {
//     use crate::parser::{
//         entry::{Reading, Task},
//         Mapper, Reader, E,
//     };

//     #[test]
//     fn reading() -> Result<(), E> {
//         let mut mapper = Mapper::new();
//         let mut reader = Reader::new(
//             include_str!("./tests/tasks.sibs").to_string(),
//             &mut mapper,
//             0,
//         );
//         while let Some(task) = Task::read(&mut reader)? {
//             println!("{task:?}");
//         }
//         println!("{:?}", reader.mapper);
//         assert!(reader.rest().trim().is_empty());
//         Ok(())
//     }
// }
