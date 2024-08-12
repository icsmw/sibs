use std::{fs, io::Write, path::PathBuf};

use crate::{
    elements::ElTarget,
    error::LinkedErr,
    reader::{Reader, E},
};

use super::{Configuration, Journal};

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
            parent: self.parent,
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
    let mut cursor = FormationCursor::default();
    let journal = Journal::init(Configuration::logs(false))?;
    let elements = Reader::read_file(filename, false, None, &journal).await?;
    let mut file = fs::OpenOptions::new().write(true).open(filename)?;
    for el in elements {
        file.write_all(format!("{}\n", el.format(&mut cursor)).as_bytes())?;
    }
    file.flush()?;
    Ok(())
}

#[cfg(test)]
pub async fn format_string(content: &str) -> Result<String, LinkedErr<E>> {
    use crate::inf::Report;

    let mut cursor = FormationCursor::default();
    let journal = Journal::init(Configuration::logs(false))?;
    let elements = match Reader::read_string(content, &journal).await {
        Ok(elements) => elements,
        Err(err) => {
            journal.report(Report::LinkedErr(err.serialize()));
            return Err(err);
        }
    };
    let mut output = String::new();
    for el in elements {
        output = format!("{output}\n{}", el.format(&mut cursor));
    }
    Ok(output)
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        functions::load,
        inf::{format_string, Configuration, Journal},
        read_string,
        reader::{chars, error::E, Reader, Sources},
    };

    #[tokio::test]
    async fn reading() {
        let formated_content = match format_string(include_str!("../../tests/formation.sibs")).await
        {
            Ok(r) => r,
            Err(_) => return,
        };
        let mut formated = Vec::new();
        read_string!(
            &Configuration::logs(false),
            &formated_content,
            |reader: &mut Reader, src: &mut Sources| {
                while let Some(el) = src.report_err_if(Element::include(
                    reader,
                    &[ElTarget::Function, ElTarget::Component],
                ))? {
                    if let Element::Function(func, _) = &el {
                        if load::NAME != func.name {
                            Err(E::OnlyImportFunctionAllowedOnRoot.by_reader(reader))?;
                        }
                        if func.args.len() != 1 {
                            Err(E::ImportFunctionInvalidArgs.by_reader(reader))?;
                        };
                        if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                            Err(E::MissedSemicolon.by_reader(reader))?;
                        }
                    }
                    formated.push(el);
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );

        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/formation.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut origin = Vec::new();
                while let Some(el) = src.report_err_if(Element::include(
                    reader,
                    &[ElTarget::Function, ElTarget::Component],
                ))? {
                    if let Element::Function(func, _) = &el {
                        if load::NAME != func.name {
                            Err(E::OnlyImportFunctionAllowedOnRoot.by_reader(reader))?;
                        }
                        if func.args.len() != 1 {
                            Err(E::ImportFunctionInvalidArgs.by_reader(reader))?;
                        };
                        if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                            Err(E::MissedSemicolon.by_reader(reader))?;
                        }
                    }
                    origin.push(el);
                }
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
                                if let (Element::Block(origin, _), Element::Block(formated, _)) =
                                    (origin.block.as_ref(), formated.block.as_ref())
                                {
                                    assert_eq!(origin.elements.len(), formated.elements.len());
                                    let origin = &origin.elements;
                                    let formated = &formated.elements;
                                    for (i, el) in origin.iter().enumerate() {
                                        assert_eq!(
                                            el.inner_to_string(),
                                            formated[i].inner_to_string()
                                        );
                                        count += 1;
                                    }
                                } else {
                                    panic!("Fail to read blocks of tasks")
                                }
                            }
                        }
                    }
                }
                assert!(count > 50);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn reading_backup() -> Result<(), LinkedErr<E>> {
        let journal = Journal::init(Configuration::logs(false))?;
        let origin =
            Reader::read_string(include_str!("../../tests/formation.sibs"), &journal).await?;

        let formated = Reader::read_string(
            &format_string(include_str!("../../tests/formation.sibs")).await?,
            &journal,
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
                        if let (Element::Block(origin, _), Element::Block(formated, _)) =
                            (origin.block.as_ref(), formated.block.as_ref())
                        {
                            assert_eq!(origin.elements.len(), formated.elements.len());
                            let origin = &origin.elements;
                            let formated = &formated.elements;
                            for (i, el) in origin.iter().enumerate() {
                                assert_eq!(el.inner_to_string(), formated[i].inner_to_string());
                                count += 1;
                            }
                        } else {
                            panic!("Fail to read blocks of tasks")
                        }
                    }
                }
            }
        }
        assert!(count > 50);
        Ok(())
    }
}
