use std::{fs, io::Write, path::PathBuf};

use crate::{
    elements::ElTarget,
    error::LinkedErr,
    inf::Context,
    reader::{read_file, E},
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
mod reading {
    use crate::{
        error::LinkedErr,
        inf::{Context, Formation, FormationCursor},
        reader::{error::E, read_file},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let target = std::env::current_dir()
            .unwrap()
            .join("./src/tests/formation.sibs");
        let mut cx = Context::create().bound(&target)?;
        let mut cursor = FormationCursor::default();
        match read_file(&mut cx, target, false).await {
            Ok(components) => {
                for component in components {
                    println!("{}", component.format(&mut cursor));
                }
            }
            Err(err) => {
                cx.sources.report_error(&err)?;
                let _ = cx.tracker.shutdown().await;
                return Err(err);
            }
        }
        Ok(())
    }
}
