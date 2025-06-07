pub mod funcs;
pub mod vars;

use crate::completion::*;

#[derive(Debug)]
pub enum CompletionMatch {
    /// `String` - name of variable
    /// `Option<Ty>` - type of variable
    Variable(String, Option<Ty>),
    /// `String` - function name
    /// `Option<String>` - function docs
    /// `Option<Ty>` - type of the first argument
    /// `Option<Ty>` - return type
    Function(String, Option<String>, Option<Ty>, Option<Ty>),
}

#[derive(Debug)]
pub struct CompletionSuggestion {
    pub target: CompletionMatch,
    pub score: u8,
}

impl CompletionSuggestion {
    pub fn repress(&mut self, rate: f32) {
        // Calculate the new score with clamping
        let adjusted_score = (self.score as f32 * rate).round() as isize;
        self.score = match adjusted_score {
            n if n >= search::MAX_SCORE as isize => search::MAX_SCORE,
            n if n <= 0 => 0,
            n => n as u8,
        };
    }
    pub fn drop(&mut self) {
        self.score = 0;
    }
}
