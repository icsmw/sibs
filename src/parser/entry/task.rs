use uuid::Uuid;

use crate::parser::{
    chars,
    entry::{Block, Reading, VariableDeclaration},
    Reader, E,
};

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub declarations: Vec<VariableDeclaration>,
    pub block: Option<Block>,
    pub uuid: Uuid,
}

impl Reading<Task> for Task {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if let Some((name, stopped_on, uuid)) =
            reader.read_until(&[chars::OPEN_BRACKET, chars::OPEN_SQ_BRACKET], false, false)?
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
            if let Some((content, uuid)) =
                reader.read_until_close(chars::OPEN_SQ_BRACKET, chars::CLOSE_SQ_BRACKET, true)?
            {
                let block = Block::read(&mut reader.inherit(content))?;
                Ok(Some(Task {
                    name,
                    declarations,
                    uuid,
                    block,
                }))
            } else {
                Err(E::FailFindTaskActions)
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        entry::{Reading, Task},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("./tests/tasks.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(task) = Task::read(&mut reader)? {
            println!("{task:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
