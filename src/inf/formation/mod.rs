use crate::elements::ElTarget;

const TAB: u8 = 4;

#[derive(Debug, Default)]
pub struct FormationCursor {
    pub offset: usize,
    pub pos: usize,
    pub parent: Option<ElTarget>,
}

impl FormationCursor {
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
    pub fn shift_right(&mut self, parent: Option<ElTarget>) -> Self {
        FormationCursor {
            offset: self.offset + 1,
            pos: 0,
            parent,
        }
    }
    pub fn reown(&mut self, parent: Option<ElTarget>) -> Self {
        FormationCursor {
            offset: self.offset,
            pos: 0,
            parent,
        }
    }
}

pub trait Formation {
    fn format(&self, cursor: &mut FormationCursor) -> String;
}

// #[cfg(test)]
// mod reading {
//     use crate::{
//         error::LinkedErr,
//         inf::{Context, Formation, FormationCursor},
//         reader::{error::E, read_file},
//     };

//     #[tokio::test]
//     async fn reading() -> Result<(), LinkedErr<E>> {
//         let target = std::env::current_dir()
//             .unwrap()
//             .join("./src/tests/formation.sibs");
//         let mut cx = Context::from_filename(&target)?;
//         let mut cursor = FormationCursor::default();
//         match read_file(&mut cx).await {
//             Ok(components) => {
//                 for component in components {
//                     println!("{}", component.format(&mut cursor));
//                 }
//             }
//             Err(err) => {
//                 cx.gen_report_from_err(&err)?;
//                 cx.post_reports();
//                 let _ = cx.tracker.shutdown().await;
//                 return Err(err);
//             }
//         }
//         Ok(())
//     }
// }
