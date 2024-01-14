use crate::{
    cli::reporter::{self, Reporter},
    reader::{
        chars,
        entry::{Block, Reading, VariableDeclaration},
        Reader, E,
    },
};
use std::fmt;

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub declarations: Vec<VariableDeclaration>,
    pub block: Option<Block>,
    pub token: usize,
}

impl Task {
    pub fn has_meta(&self) -> bool {
        self.block
            .as_ref()
            .map(|b| b.meta.is_some())
            .unwrap_or(false)
    }
}

impl Reading<Task> for Task {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if let Some((mut name, stopped_on)) = reader
            .until()
            .char(&[&chars::OPEN_BRACKET, &chars::OPEN_SQ_BRACKET])
        {
            name = name.trim().to_string();
            if stopped_on == chars::OPEN_BRACKET {
                reader.move_to().next();
            }
            if !Reader::is_ascii_alphabetic_and_alphanumeric(
                &name,
                &[&chars::UNDERSCORE, &chars::DASH],
            ) {
                Err(E::InvalidTaskName)?
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
                if !token.bound.rest().trim().is_empty() {
                    Err(E::InvalidTaskArguments)?
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
                if reader.move_to().char(&[&chars::SEMICOLON]).is_some() {
                    reader.move_to().next();
                }
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

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} {}",
            self.name,
            if self.declarations.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.declarations
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<String>>()
                        .join("; ")
                )
            },
            self.block
                .as_ref()
                .map(|b| format!("{b};"))
                .unwrap_or_default()
        )
    }
}

impl reporter::Display for Task {
    fn display(&self, reporter: &mut Reporter) {
        reporter.bold(format!("{}[{}]", reporter.offset(), self.name));
        println!();
        reporter.step_right();
        reporter.print(format!(
            "{}USAGE: {}{}{}",
            reporter.offset(),
            self.name,
            if self.declarations.is_empty() {
                ""
            } else {
                " "
            },
            self.declarations
                .iter()
                .map(reporter::Display::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        ));
        println!();
        if let Some(block) = self.block.as_ref() {
            block.display(reporter);
        }
        reporter.step_left();
    }
}

#[cfg(test)]
mod test_tasks {
    use crate::reader::{
        entry::{Reading, Task},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/normal/tasks.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Task::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 6);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("./tests/error/tasks.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Task::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}
