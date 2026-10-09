mod array;
mod boolean;
mod closure;
mod error;
mod interpolated_string;
mod number;
mod primitive_string;

use crate::*;

impl Lint for Value {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        match self {
            Value::Array(n) => n.lint(md, ctx),
            Value::Boolean(n) => n.lint(md, ctx),
            Value::Error(n) => n.lint(md, ctx),
            Value::InterpolatedString(n) => n.lint(md, ctx),
            Value::Number(n) => n.lint(md, ctx),
            Value::PrimitiveString(n) => n.lint(md, ctx),
            Value::Closure(n) => n.lint(md, ctx),
        }
    }
}
