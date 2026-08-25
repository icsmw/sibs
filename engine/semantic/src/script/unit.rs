use std::path::Path;

use asttree::*;
use parser::*;

use crate::*;

#[derive(Debug)]
pub struct Script {
    parser: Parser,
    anchor: Anchor,
    scx: SemanticCx,
    source: ScriptSource,
    options: ScriptOptions,
}

impl Script {
    pub fn from_file<P: AsRef<Path>>(path: P, options: ScriptOptions) -> Result<Self, ScriptError> {
        Self::read(ScriptSource::file(path.as_ref()), options)
    }

    pub fn from_text<S: ToString>(content: S, options: ScriptOptions) -> Result<Self, ScriptError> {
        Self::read(ScriptSource::text(content), options)
    }

    fn read(source: ScriptSource, options: ScriptOptions) -> Result<Self, ScriptError> {
        let parser = source.parser(options.resilience)?;
        let anchor = match Anchor::read(&parser) {
            Ok(Some(anchor)) => anchor,
            Ok(None) => return Err(ScriptError::FailExtractAnchorNodeFrom(source.to_string())),
            Err(err) => {
                if !options.resilience {
                    return Err(ScriptError::Parsing(err));
                }
                let mut diagnostics = vec![DiagnosticError::convert(err)];
                diagnostics.extend(
                    parser
                        .errs
                        .borrow_mut()
                        .drain()
                        .into_iter()
                        .map(DiagnosticError::convert),
                );
                return Err(ScriptError::Diagnostics(ScriptDiagnostics::new(
                    parser,
                    diagnostics,
                )));
            }
        };
        let mut diagnostics = parser
            .errs
            .borrow_mut()
            .drain()
            .into_iter()
            .map(DiagnosticError::convert)
            .collect::<Vec<_>>();
        parser.bind(anchor.nodes())?;

        let mut scx = SemanticCx::new(options.resilience);
        functions::register(&mut scx.fns.efns)?;
        if let Err(err) = anchor.initialize(&mut scx) {
            if !options.resilience {
                return Err(ScriptError::Semantic(err));
            }
            diagnostics.push(DiagnosticError::convert(err));
        }
        if let Err(err) = anchor.infer_type(&mut scx) {
            if !options.resilience {
                return Err(ScriptError::Semantic(err));
            }
            diagnostics.push(DiagnosticError::convert(err));
        }
        if let Err(err) = anchor.finalize(&mut scx) {
            if !options.resilience {
                return Err(ScriptError::Semantic(err));
            }
            diagnostics.push(DiagnosticError::convert(err));
        }
        diagnostics.extend(scx.errs.drain().into_iter().map(DiagnosticError::convert));
        if !diagnostics.is_empty() {
            return Err(ScriptError::Diagnostics(ScriptDiagnostics::new(
                parser,
                diagnostics,
            )));
        }

        Ok(Self {
            parser,
            anchor,
            scx,
            source,
            options,
        })
    }

    pub fn parser(&self) -> &Parser {
        &self.parser
    }

    pub fn anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn semantic(&self) -> &SemanticCx {
        &self.scx
    }

    pub fn source(&self) -> &ScriptSource {
        &self.source
    }

    pub fn options(&self) -> ScriptOptions {
        self.options
    }

    pub fn into_inner(self) -> ScriptInner {
        ScriptInner {
            parser: self.parser,
            anchor: self.anchor,
            scx: self.scx,
            source: self.source,
            options: self.options,
        }
    }
}

#[derive(Debug)]
pub struct ScriptInner {
    pub parser: Parser,
    pub anchor: Anchor,
    pub scx: SemanticCx,
    pub source: ScriptSource,
    pub options: ScriptOptions,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_valid_text_script() {
        let script = Script::from_text(
            r#"
            component my_component() {
                task task_a() {
                    true;
                }
            };
            "#,
            ScriptOptions::strict(),
        )
        .expect("script is prepared");

        assert!(script.anchor().get_component("my_component").is_some());
    }

    #[test]
    fn resilient_returns_collected_semantic_diagnostics() {
        let err = Script::from_text(
            r#"
            component my_component() {
                task task_a() {
                    let value: num = true;
                    let other: bool = 5;
                    true;
                }
            };
            "#,
            ScriptOptions::resilient(),
        )
        .expect_err("script has diagnostics");

        let ScriptError::Diagnostics(diagnostics) = err else {
            panic!("expected collected diagnostics");
        };
        assert!(
            (&diagnostics).into_iter().count() == 2,
            "expected multiple diagnostics, got {diagnostics:?}"
        );
        let first = (&diagnostics)
            .into_iter()
            .next()
            .expect("diagnostic exists");
        assert!(!first.to_string().is_empty());
        assert!(!first.report().expect("diagnostic report").is_empty());
        assert!(!first.inner().e.to_string().is_empty());
        let output = diagnostics.to_string();
        assert!(!output.is_empty());
    }

    #[test]
    fn diagnostics_iterator_exposes_error_views() {
        let err = Script::from_text(
            r#"
            component my_component() {
                task task_a() {
                    let value: num = true;
                    let other: bool = 5;
                    true;
                }
            };
            "#,
            ScriptOptions::resilient(),
        )
        .expect_err("script has diagnostics");

        let ScriptError::Diagnostics(diagnostics) = err else {
            panic!("expected collected diagnostics");
        };

        let items = (&diagnostics).into_iter().collect::<Vec<_>>();
        assert_eq!(items.len(), 2);

        for item in items {
            let message = item.to_string();
            let report = item.report().expect("diagnostic report");
            let inner = item.inner();

            assert!(!message.is_empty());
            assert!(report.contains(&message));
            assert_eq!(message, inner.e.to_string());
            assert!(inner.link.from.abs < inner.link.to.abs);
            assert!(report.contains("^"));
        }
    }

    #[test]
    fn strict_returns_first_semantic_error() {
        let err = Script::from_text(
            r#"
            component my_component() {
                task task_a() {
                    let value: num = true;
                    let other: bool = 5;
                    true;
                }
            };
            "#,
            ScriptOptions::strict(),
        )
        .expect_err("script fails fast");

        assert!(
            matches!(err, ScriptError::Semantic(_)),
            "expected first semantic error, got {err:?}"
        );
    }
}
