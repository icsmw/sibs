use crate::reader::{
    chars,
    entry::{Each, Function, If, Meta, Optional, Reading, Reference, VariableAssignation},
    Reader, E,
};

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
    meta: Option<Meta>,
    elements: Vec<Element>,
    index: usize,
}

impl Reading<Block> for Block {
    fn read(reader: &mut Reader) -> Result<Option<Block>, E> {
        let mut elements: Vec<Element> = vec![];
        let mut meta: Option<Meta> = None;
        let from = reader.pos;
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
            if let Some((inner, _, _)) = reader.read_until(&[chars::SEMICOLON], true, false)? {
                let mut inner = reader.inherit(inner);
                if let Some(el) = Function::read(&mut inner)? {
                    elements.push(Element::Function(el));
                } else if let Some(el) = Optional::read(&mut inner)? {
                    elements.push(Element::Optional(el));
                } else if let Some(el) = VariableAssignation::read(&mut inner)? {
                    elements.push(Element::VariableAssignation(el));
                } else if let Some(el) = Each::read(&mut inner)? {
                    elements.push(Element::Each(el));
                } else {
                    elements.push(Element::Command(inner.rest().to_string()));
                }
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
                index: reader.get_index_until_current(from),
            })
        })
    }
}

#[cfg(test)]
mod blocks {
    use crate::reader::{
        entry::{Block, Reading},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}",
                include_str!("./tests/if.sibs"),
                include_str!("./tests/variable_assignation.sibs"),
                include_str!("./tests/function.sibs"),
                include_str!("./tests/optional.sibs"),
                include_str!("./tests/each.sibs"),
                include_str!("./tests/refs.sibs")
            ),
            &mut mapper,
            0,
        );
        while let Some(task) = Block::read(&mut reader)? {
            println!("{task:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
