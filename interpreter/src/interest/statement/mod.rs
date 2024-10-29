mod r#break;
mod r#return;

pub use r#break::*;
pub use r#return::*;

use crate::*;
use lexer::Token;

impl Interest for Statement {
    fn interest_in_token(token: &Token, nodes: &Nodes) -> bool {
        for candidate in StatementId::as_vec().into_iter() {
            if match candidate {
                StatementId::Break => Break::interest_in_token(token, nodes),
                StatementId::Return => Return::interest_in_token(token, nodes),
            } {
                return true;
            }
        }
        false
    }
}
