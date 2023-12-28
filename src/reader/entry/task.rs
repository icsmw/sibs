use crate::reader::{
    chars,
    entry::{Block, Reading, VariableDeclaration},
    Reader, E,
};

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub declarations: Vec<VariableDeclaration>,
    pub block: Option<Block>,
    pub token: usize,
}

impl Reading<Task> for Task {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if let Some((name, stopped_on)) = reader
            .until()
            .char(&[&chars::OPEN_BRACKET, &chars::OPEN_SQ_BRACKET])
        {
            if stopped_on == chars::OPEN_BRACKET {
                reader.move_to().next();
            }
            let declarations: Vec<VariableDeclaration> = if stopped_on == chars::OPEN_SQ_BRACKET {
                vec![]
            } else if reader.until().char(&[&chars::CLOSE_BRACKET]).is_some() {
                reader.move_to().next();
                let mut declarations: Vec<VariableDeclaration> = vec![];
                let mut token = reader.token()?;
                while let Some(variable_declaration) = VariableDeclaration::read(&mut token.bound)?
                {
                    declarations.push(variable_declaration);
                }
                declarations
            } else {
                Err(E::NoTaskArguments)?
            };
            if reader
                .group()
                .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                .is_some()
            {
                let mut token = reader.token()?;
                let block = Block::read(&mut token.bound)?;
                Ok(Some(Task {
                    name,
                    declarations,
                    token: token.id,
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
mod test_tasks {
    use crate::reader::{
        entry::{Reading, Task},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/tasks.sibs").to_string());
        let mut count = 0;
        while let Some(task) = Task::read(&mut reader)? {
            println!("{task:?}");
            count += 1;
        }
        assert_eq!(count, 6);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
