use crate::{
    elements::{ElementId, IfSubsequence},
    inf::{Formation, FormationCursor},
};

impl Formation for IfSubsequence {
    fn elements_count(&self) -> usize {
        self.subsequence.iter().map(|s| s.elements_count()).sum()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        if self.elements_count() > cursor.max_elements()
            || self.to_string().len() > cursor.max_len()
        {
            let mut inner = cursor.reown(Some(ElementId::IfSubsequence));
            self.subsequence
                .chunks(2)
                .enumerate()
                .map(|(i, pair)| {
                    format!(
                        "{}{}{}",
                        if i == 0 {
                            cursor.offset_as_string_if(&[ElementId::Block])
                        } else {
                            String::new()
                        },
                        pair[0].format(&mut inner),
                        if pair.len() > 1 {
                            format!(
                                "\n{}{}",
                                cursor.offset_as_string(),
                                pair[1].format(&mut inner)
                            )
                        } else {
                            String::new()
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join("")
        } else {
            format!("{}{self}", cursor.offset_as_string_if(&[ElementId::Block]))
        }
    }
}
