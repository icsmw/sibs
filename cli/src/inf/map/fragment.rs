use regex::Regex;

use crate::elements::ElementRef;

#[derive(Debug, Clone)]
pub struct Fragment {
    pub content: String,
    #[allow(unused)]
    pub lined: String,
    /// Start position in whole content of file
    #[allow(unused)]
    pub from: usize,
    /// Length of fragment
    #[allow(unused)]
    pub len: usize,
    /// End position in whole content of file
    #[allow(unused)]
    pub to: usize,
    /// Start line in whole content of file
    pub from_ln: usize,
    /// End line in whole content of file
    pub to_ln: usize,
    /// Start position in start line
    pub from_pos: usize,
    /// End position in end line
    pub to_pos: usize,
    /// Type of element
    pub el: Option<ElementRef>,
}

impl Fragment {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        el: Option<ElementRef>,
        content: String,
        from: usize,
        len: usize,
        from_ln: usize,
        to_ln: usize,
        from_pos: usize,
        to_pos: usize,
    ) -> Self {
        let lined = Regex::new(r"[\n\r]\s*")
            .expect("Regex [\\n\\r]\\s* should be constructed")
            .replace_all(&content, "")
            .to_string();
        Fragment {
            content,
            lined,
            from,
            len,
            to: from + len,
            from_ln,
            to_ln,
            from_pos,
            to_pos,
            el,
        }
    }
}
