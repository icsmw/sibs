use std::{fs, io::Write, path::PathBuf};

use crate::{
    elements::ElTarget,
    error::LinkedErr,
    inf::Context,
    reader::{read_file, read_string, E},
};

const TAB: u8 = 4;

const MAX_FORMATED_LINE_LEN: usize = 120;
const MAX_INLINE_INJECTIONS: usize = 6;
const MAX_ARGS: usize = 4;
const MAX_ITEMS: usize = 6;
const MAX_ELEMENTS: usize = 4;

#[derive(Debug, Default)]
pub struct FormationCursor {
    pub offset: usize,
    pub parent: Option<ElTarget>,
}

impl FormationCursor {
    pub fn max_len(&self) -> usize {
        let offset = self.offset * TAB as usize;
        if MAX_FORMATED_LINE_LEN < offset {
            0
        } else {
            MAX_FORMATED_LINE_LEN - offset
        }
    }
    pub fn max_inline_injections(&self) -> usize {
        MAX_INLINE_INJECTIONS
    }
    pub fn max_args(&self) -> usize {
        MAX_ARGS
    }
    pub fn max_items(&self) -> usize {
        MAX_ITEMS
    }
    pub fn max_elements(&self) -> usize {
        MAX_ELEMENTS
    }
    pub fn offset_as_string(&self) -> String {
        " ".repeat(TAB as usize).repeat(self.offset)
    }
    pub fn offset_as_string_if(&self, targets: &[ElTarget]) -> String {
        if let Some(parent) = self.parent.as_ref() {
            if targets.contains(parent) {
                return " ".repeat(TAB as usize).repeat(self.offset);
            }
        }
        String::new()
    }
    pub fn right(&mut self) -> Self {
        FormationCursor {
            offset: self.offset + 1,
            parent: self.parent.clone(),
        }
    }
    pub fn reown(&mut self, parent: Option<ElTarget>) -> Self {
        FormationCursor {
            offset: self.offset,
            parent,
        }
    }
}

pub trait Formation {
    fn format(&self, cursor: &mut FormationCursor) -> String;
    fn elements_count(&self) -> usize {
        1
    }
}

pub async fn format_file(filename: &PathBuf) -> Result<(), LinkedErr<E>> {
    let mut cx = Context::create().bound(filename)?;
    let mut cursor = FormationCursor::default();
    let elements = read_file(&mut cx, filename.clone(), false).await?;
    let mut file = fs::OpenOptions::new().write(true).open(filename)?;
    for el in elements {
        file.write_all(format!("{}\n", el.format(&mut cursor)).as_bytes())?;
    }
    file.flush()?;
    Ok(())
}

#[cfg(test)]
pub async fn format_string(content: &str) -> Result<String, LinkedErr<E>> {
    let mut cx = Context::create().unbound()?;
    let mut cursor = FormationCursor::default();
    let elements = read_string(&mut cx, content).await?;
    let mut output = String::new();
    for el in elements {
        output = format!("{output}\n{}", el.format(&mut cursor));
    }
    Ok(output)
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Element,
        error::LinkedErr,
        inf::{format_string, Context},
        reader::{error::E, read_string},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let origin = read_string(
            &mut Context::create().unbound()?,
            include_str!("../../tests/formation.sibs"),
        )
        .await?;
        let formated = read_string(
            &mut Context::create().unbound()?,
            &format_string(include_str!("../../tests/formation.sibs")).await?,
        )
        .await?;
        assert_eq!(origin.len(), formated.len());
        let mut count: usize = 0;
        for (i, el) in origin.iter().enumerate() {
            assert_eq!(el.el_target(), formated[i].el_target());
            if let (Element::Component(origin, _), Element::Component(formated, _)) =
                (el, &formated[i])
            {
                assert_eq!(origin.elements.len(), formated.elements.len());
                for (i, el) in origin.elements.iter().enumerate() {
                    assert_eq!(el.el_target(), formated.elements[i].el_target());
                    if let (Element::Task(origin, _), Element::Task(formated, _)) =
                        (el, &formated.elements[i])
                    {
                        assert_eq!(origin.block.elements.len(), formated.block.elements.len());
                        let origin = &origin.block.elements;
                        let formated = &formated.block.elements;
                        for (i, el) in origin.iter().enumerate() {
                            assert_eq!(el.to_string(), formated[i].to_string());
                            count += 1;
                        }
                    }
                }
            }
        }
        assert!(count > 50);
        Ok(())
    }
}
