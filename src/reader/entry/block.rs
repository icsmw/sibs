use crate::reader::{
    chars,
    entry::{Each, Function, If, Meta, Optional, Reading, Reference, VariableAssignation},
    Reader, E,
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
    Command(String),
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
                elements.push(Element::Command(cmd));
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
        write!(f, "[]")
    }
}

#[cfg(test)]
mod test_blocks {
    use crate::reader::{
        entry::{Block, Reading},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            include_str!("./tests/if.sibs"),
            include_str!("./tests/variable_assignation.sibs"),
            include_str!("./tests/function.sibs"),
            include_str!("./tests/optional.sibs"),
            include_str!("./tests/each.sibs"),
            include_str!("./tests/refs.sibs")
        ));
        while let Some(task) = Block::read(&mut reader)? {
            println!("{task:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
