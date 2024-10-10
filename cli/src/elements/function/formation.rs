use crate::{
    elements::{function::Function, ElementRef},
    inf::{Formation, FormationCursor},
};

impl Formation for Function {
    fn elements_count(&self) -> usize {
        self.args.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        // fn formated(func: &Function, cursor: &mut FormationCursor) -> String {
        //     format!(
        //         "{}({}{}",
        //         func.name,
        //         func.args
        //             .iter()
        //             .map(|arg| format!(
        //                 "\n{}{}",
        //                 cursor.right().offset_as_string(),
        //                 arg.format(&mut cursor.reown(Some(ElementRef::Function)).right())
        //             ))
        //             .collect::<Vec<String>>()
        //             .join(", "),
        //         if func.args.is_empty() {
        //             ")".to_string()
        //         } else {
        //             format!("\n{})", cursor.offset_as_string_if(&[ElementRef::Block]))
        //         }
        //     )
        // }
        let output = format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block, ElementRef::Component]),
            self
        );
        format!(
            "{output}{}",
            if cursor.parent.is_none() { ";\n" } else { "" }
        )
    }
}
