use crate::{
    elements::Comment,
    inf::{Formation, FormationCursor},
};

impl Formation for Comment {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut collected = String::new();
        let mut lines: Vec<String> = Vec::new();
        for word in self.comment.split_whitespace() {
            collected = format!("{collected} {word}");
            if collected.len() >= cursor.max_len() {
                lines.push(collected);
                collected = String::new();
            }
        }
        if !collected.trim().is_empty() {
            lines.push(collected);
        }
        lines
            .iter()
            .map(|l| format!("{}// {l}", cursor.offset_as_string()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
