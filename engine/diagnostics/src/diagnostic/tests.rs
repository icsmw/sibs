use super::*;
use lexer::{Lexer, LinkedPosition, TextPosition};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
enum TestError {
    #[error("Unknown variable")]
    UnknownVariable,
    #[error("Invalid expression")]
    InvalidExpression,
}

impl ErrorCode for TestError {
    fn code(&self) -> &'static str {
        match self {
            Self::UnknownVariable => "TEST00001",
            Self::InvalidExpression => "TEST00002",
        }
    }
    fn src(&self) -> ErrorSource {
        ErrorSource::Semantic
    }
}

#[test]
fn errors_preserve_order_deduplicate_and_can_be_reused_after_drain() {
    let mut errors = Errors::default();
    for err in [
        TestError::UnknownVariable,
        TestError::InvalidExpression,
        TestError::UnknownVariable,
    ] {
        errors.push(LinkedErr::unlinked(err));
    }

    assert_eq!(errors.slice().len(), 2);
    assert!(matches!(
        errors.slice().first().unwrap().e,
        TestError::UnknownVariable
    ));
    assert!(matches!(errors.slice()[1].e, TestError::InvalidExpression));

    let drained = errors.drain();
    assert_eq!(drained.len(), 2);
    assert!(matches!(drained[0].e, TestError::UnknownVariable));
    assert!(matches!(drained[1].e, TestError::InvalidExpression));
    assert!(errors.is_empty());
    assert!(errors.slice().is_empty());

    errors.push(LinkedErr::unlinked(TestError::UnknownVariable));
    assert_eq!(errors.slice().len(), 1);
}

#[test]
fn extracted_error_can_be_added_again_at_the_end() {
    let mut errors = Errors::default();
    errors.push(LinkedErr::unlinked(TestError::UnknownVariable));
    errors.push(LinkedErr::unlinked(TestError::InvalidExpression));

    let first = errors.take_first().unwrap();
    assert!(matches!(first.e, TestError::UnknownVariable));
    assert_eq!(errors.slice().len(), 1);
    errors.push(first);
    assert_eq!(errors.slice().len(), 2);
    assert!(matches!(
        errors.take_first().unwrap().e,
        TestError::InvalidExpression
    ));
    assert!(matches!(
        errors.take_first().unwrap().e,
        TestError::UnknownVariable
    ));
    assert!(errors.take_first().is_none());
    assert!(errors.is_empty());
}

#[test]
fn transforming_errors_updates_deduplication_keys() {
    let mut errors = Errors::default();
    errors.push(LinkedErr::unlinked(TestError::UnknownVariable));
    errors.push(LinkedErr::unlinked(TestError::InvalidExpression));

    let mut errors =
        errors.transform(|err| LinkedErr::by_link(TestError::InvalidExpression, err.link));
    assert_eq!(errors.slice().len(), 1);
    assert!(matches!(
        errors.slice().first().unwrap().e,
        TestError::InvalidExpression
    ));

    errors.push(LinkedErr::unlinked(TestError::InvalidExpression));
    assert_eq!(errors.slice().len(), 1);
    errors.push(LinkedErr::unlinked(TestError::UnknownVariable));
    assert_eq!(errors.slice().len(), 2);
}

#[test]
fn reports_single_line_error_with_source_and_marker() {
    let content = "let value = missing;\n";
    let src = Uuid::new_v4();
    let err = LinkedErr::by_link(
        TestError::UnknownVariable,
        LinkedPosition::new(
            TextPosition {
                abs: 12,
                ln: 0,
                col: 12,
            },
            TextPosition {
                abs: 19,
                ln: 0,
                col: 19,
            },
            &src,
        ),
    );
    let mut errors = Errors::default();
    errors.push(err);
    let diagnostics = Diagnostics::new(
        CodeSources::unbound(content, &src),
        Tokens::default(),
        errors,
    );
    let mut output = Vec::new();
    diagnostics.errors()[0]
        .report(&diagnostics, &mut output)
        .unwrap();
    let report = String::from_utf8(output).unwrap();
    assert!(report.contains("let value = missing;"), "{report}");
    assert!(report.contains("^^^^^^^"), "{report}");
    assert_eq!(report.matches("Unknown variable").count(), 1);
}

#[test]
fn reports_multiline_error_with_all_affected_lines() {
    let content = "before\nfirst\nsecond\nafter\n";
    let src = Uuid::new_v4();
    let err = LinkedErr::by_link(
        TestError::InvalidExpression,
        LinkedPosition::new(
            TextPosition {
                abs: 7,
                ln: 1,
                col: 0,
            },
            TextPosition {
                abs: 19,
                ln: 2,
                col: 6,
            },
            &src,
        ),
    );
    let mut errors = Errors::default();
    errors.push(err);
    let diagnostics = Diagnostics::new(
        CodeSources::unbound(content, &src),
        Tokens::default(),
        errors,
    );
    let mut output = Vec::new();
    diagnostics
        .err(&diagnostics.errors()[0], &mut output)
        .unwrap();
    let report = String::from_utf8(output).unwrap();
    assert!(report.contains("first"), "{report}");
    assert!(report.contains("second"), "{report}");
    assert_eq!(report.matches('>').count(), 2, "{report}");
    assert_eq!(report.matches("Invalid expression").count(), 1);
}

#[test]
fn transforming_errors_preserves_sources_tokens_and_positions() {
    let content = "missing";
    let mut lexer = Lexer::new(content, 0);
    let tokens = lexer.read().unwrap().tokens;
    let token = tokens
        .iter()
        .find(|token| token.to_string() == content)
        .unwrap();
    let position = token.pos.from.abs;
    let err = LinkedErr::token(TestError::UnknownVariable, token);
    let mut errors = Errors::default();
    errors.push(err);
    let diagnostics = Diagnostics::new(
        CodeSources::unbound(content, &lexer.uuid),
        Tokens::with(tokens),
        errors,
    );
    let diagnostics =
        diagnostics.transform(|err| LinkedErr::by_link(TestError::InvalidExpression, err.link));

    assert_eq!(diagnostics.errors().len(), 1);
    let err = &diagnostics.errors()[0];
    assert!(matches!(err.e, TestError::InvalidExpression));
    assert_eq!(err.link.src, lexer.uuid);
    assert_eq!(err.link.from.abs, position);
    assert_eq!(
        diagnostics
            .sources()
            .get_content(&lexer.uuid)
            .unwrap()
            .as_deref(),
        Some(content)
    );
    assert_eq!(
        diagnostics
            .get_token_by_pos(position)
            .unwrap()
            .0
            .to_string(),
        content
    );
}
