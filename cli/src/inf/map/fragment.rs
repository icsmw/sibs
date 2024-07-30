use regex::Regex;

#[derive(Debug, Clone)]
pub struct Fragment {
    pub content: String,
    #[allow(unused)]
    pub lined: String,
    #[allow(unused)]
    pub from: usize,
    #[allow(unused)]
    pub len: usize,
    #[allow(unused)]
    pub to: usize,
    pub from_ln: usize,
    pub to_ln: usize,
}

impl Fragment {
    pub fn new(content: String, from: usize, len: usize, from_ln: usize, to_ln: usize) -> Self {
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
        }
    }
}
