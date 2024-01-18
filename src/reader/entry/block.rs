use crate::{
    cli,
    inf::{
        context::Context,
        runner::{self, Runner},
        term::{self, Term},
    },
    reader::{
        chars,
        entry::{
            Command, Component, Each, Function, If, Meta, Optional, Reading, Reference,
            VariableAssignation,
        },
        Reader, E,
    },
};
use std::fmt;

#[derive(Debug)]
pub enum Element {
    Function(Function),
    If(If),
    Each(Each),
    VariableAssignation(VariableAssignation),
    Optional(Optional),
    Reference(Reference),
    Command(Command),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Command(v) => v.to_string(),
                Self::Function(v) => v.to_string(),
                Self::If(v) => v.to_string(),
                Self::Each(v) => v.to_string(),
                Self::VariableAssignation(v) => v.to_string(),
                Self::Optional(v) => v.to_string(),
                Self::Reference(v) => v.to_string(),
            }
        )
    }
}

impl Runner for Element {
    fn run(
        &self,
        components: &[Component],
        args: &[String],
        context: &mut Context,
    ) -> Result<runner::Return, cli::error::E> {
        match self {
            Self::Command(v) => v.run(components, args, context),
            Self::Function(v) => v.run(components, args, context),
            Self::If(v) => v.run(components, args, context),
            Self::Each(v) => v.run(components, args, context),
            Self::VariableAssignation(v) => v.run(components, args, context),
            Self::Optional(v) => v.run(components, args, context),
            Self::Reference(v) => v.run(components, args, context),
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub meta: Option<Meta>,
    pub elements: Vec<Element>,
    pub token: usize,
}

impl Reading<Block> for Block {
    fn read(reader: &mut Reader) -> Result<Option<Block>, E> {
        let mut elements: Vec<Element> = vec![];
        let mut meta: Option<Meta> = None;
        while !reader.rest().trim().is_empty() {
            if let Some(md) = Meta::read(reader)? {
                meta = Some(md);
                continue;
            }
            if let Some(el) = If::read(reader)? {
                elements.push(Element::If(el));
                continue;
            }
            if let Some(el) = Optional::read(reader)? {
                elements.push(Element::Optional(el));
                continue;
            }
            if let Some(el) = VariableAssignation::read(reader)? {
                elements.push(Element::VariableAssignation(el));
                continue;
            }
            if let Some(el) = Each::read(reader)? {
                elements.push(Element::Each(el));
                continue;
            }
            if let Some(el) = Reference::read(reader)? {
                elements.push(Element::Reference(el));
                continue;
            }
            if let Some(el) = Function::read(reader)? {
                elements.push(Element::Function(el));
                continue;
            }
            if let Some((cmd, _)) = reader.until().char(&[&chars::SEMICOLON]) {
                reader.move_to().next();
                elements.push(Element::Command(Command::new(cmd, reader.token()?.id)));
            } else {
                break;
            }
        }
        Ok(if elements.is_empty() {
            None
        } else {
            Some(Block {
                elements,
                meta,
                token: reader.token()?.id,
            })
        })
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[\n{}{}{}]",
            self.meta
                .as_ref()
                .map(|meta| {
                    format!(
                        "{}{}",
                        meta.inner
                            .iter()
                            .map(|v| format!("/// {v}"))
                            .collect::<Vec<String>>()
                            .join("\n"),
                        if meta.inner.is_empty() { "" } else { "\n" }
                    )
                })
                .unwrap_or_default(),
            self.elements
                .iter()
                .map(|el| format!(
                    "{el}{}",
                    match el {
                        Element::Function(_) | Element::Command(_) => ";",
                        _ => "",
                    }
                ))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.elements.is_empty() { "" } else { "\n" }
        )
    }
}

impl term::Display for Block {
    fn display(&self, term: &mut Term) {
        if let Some(meta) = self.meta.as_ref() {
            meta.display(term);
        }
    }
}

impl Runner for Block {
    fn run(
        &self,
        components: &[Component],
        args: &[String],
        context: &mut Context,
    ) -> Result<runner::Return, cli::error::E> {
        let mut output: runner::Return = None;
        for element in self.elements.iter() {
            output = element.run(components, args, context)?;
        }
        Ok(output)
    }
}

#[cfg(test)]
mod test_blocks {
    use crate::reader::{
        entry::{Block, Reading},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            include_str!("./tests/normal/if.sibs"),
            include_str!("./tests/normal/variable_assignation.sibs"),
            include_str!("./tests/normal/function.sibs"),
            include_str!("./tests/normal/optional.sibs"),
            include_str!("./tests/normal/each.sibs"),
            include_str!("./tests/normal/refs.sibs")
        ));
        while let Some(entity) = Block::read(&mut reader)? {
            assert_eq!(
                format!("[{}]", tests::trim(reader.recent())),
                tests::trim(&entity.to_string())
            );
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
