mod bindings;
mod parsing;

use crate::*;

pub(super) struct Files(PathBuf);
impl Files {
    pub(super) fn new() -> Self {
        let path = std::env::temp_dir().join(format!("sibs-parser-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    pub(super) fn write(&self, name: &str, content: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub(super) fn parser(source: &str) -> Parser {
    let mut lexer = Lexer::new(source, 0);
    Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, source, false)
}
