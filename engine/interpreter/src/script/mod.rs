mod error;
mod options;
mod source;

pub use error::E as ScriptError;
pub use options::*;
pub use source::*;

use std::path::Path;

use asttree::*;
use parser::*;

use crate::*;

/// Prepares a script for execution, retaining analysis results in the context.
/// Resilient analysis collects errors instead of stopping at the first one,
/// but both modes return `ScriptError::NotExecutable` when diagnostics contain errors.
#[derive(Debug)]
pub struct Script {}

impl Script {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        options: ScriptOptions,
        ctx: &mut InterContext,
    ) -> Result<(), ScriptError> {
        Self::read(ScriptSource::file(path.as_ref()), options, ctx)
    }

    pub fn from_text<S: ToString>(
        content: S,
        options: ScriptOptions,
        ctx: &mut InterContext,
    ) -> Result<(), ScriptError> {
        Self::read(ScriptSource::text(content), options, ctx)
    }

    fn read(
        source: ScriptSource,
        options: ScriptOptions,
        ctx: &mut InterContext,
    ) -> Result<(), ScriptError> {
        fn set_diagnostics(parser: Parser, ctx: &mut InterContext) -> Result<(), ScriptError> {
            let diagnostics: Diagnostics<ParserError> = parser.try_into()?;
            ctx.set_diagnostics(diagnostics.transform(DiagnosticError::from_parser_err));
            Ok(())
        }
        fn push_err(
            ctx: &mut InterContext,
            err: LinkedErr<DiagnosticError>,
        ) -> Result<(), ScriptError> {
            let Some(diagnostics) = ctx.get_diagnostics_mut() else {
                return Err(ScriptError::ContextError);
            };
            diagnostics.push_err(err);
            Ok(())
        }
        if !ctx.is_empty() {
            return Err(ScriptError::UsedContext);
        }
        let parser = source.parser(options.resilience)?;
        let anchor = LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Anchor]));

        let flush = parser.flush().map_err(ScriptError::Parser);
        set_diagnostics(parser, ctx)?;
        flush?;

        let anchor = match anchor {
            Ok(Some(anchor)) => anchor,
            Ok(None) => {
                return Err(ScriptError::FailExtractAnchorNodeFrom(source.to_string()));
            }
            Err(err) => {
                push_err(ctx, DiagnosticError::from_parser_err(err))?;
                return Err(ScriptError::NotExecutable);
            }
        };

        ctx.set_anchor(anchor);
        ctx.set_semantic_cx(SemanticCx::new(options.resilience));

        let (Some(anchor), Some(scx), Some(diagnostics)) = ctx.get_script_ctx() else {
            unreachable!("Diagnostics has been setted")
        };

        functions::register(&mut scx.fns.efns)?;

        if let Err(err) = anchor.initialize(scx) {
            diagnostics.push_err(DiagnosticError::from_semantic_err(err));
            if !options.resilience {
                return Err(ScriptError::NotExecutable);
            }
        }
        if let Err(err) = anchor.infer_type(scx) {
            diagnostics.push_err(DiagnosticError::from_semantic_err(err));
            if !options.resilience {
                return Err(ScriptError::NotExecutable);
            }
        }
        if let Err(err) = anchor.finalize(scx) {
            diagnostics.push_err(DiagnosticError::from_semantic_err(err));
            if !options.resilience {
                return Err(ScriptError::NotExecutable);
            }
        }
        for err in scx.errs.drain(..) {
            diagnostics.push_err(DiagnosticError::from_semantic_err(err));
        }
        if diagnostics.errors().is_empty() {
            Ok(())
        } else {
            Err(ScriptError::NotExecutable)
        }
    }
}

#[cfg(test)]
mod tests;
