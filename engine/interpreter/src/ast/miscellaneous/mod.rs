mod comment;
mod meta;

use crate::*;

impl Interpret for Miscellaneous {
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.interpret(env),
            Miscellaneous::Meta(n) => n.interpret(env),
        }
    }
}
