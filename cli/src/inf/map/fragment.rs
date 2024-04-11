use regex::Regex;

#[derive(Debug)]
pub struct Fragment {
    pub content: String,
    pub lined: String,
    pub from: usize,
    pub len: usize,
    pub to: usize,
}

impl Fragment {
    pub fn new(content: String, from: usize, len: usize) -> Self {
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
        }
    }
}
