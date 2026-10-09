use crate::*;
use diagnostics::*;

impl ErrorCode for E {
    fn code(&self) -> &'static str {
        match self {
            Self::MissingComponentDocs(..) => "00001",
            Self::MissingTaskDocs(..) => "00002",
            Self::MissingTaskArgumentDocs(..) => "00003",
        }
    }
    fn src(&self) -> ErrorSource {
        ErrorSource::Lint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documentation_checks_have_distinct_lint_codes() {
        let errors = [
            E::MissingComponentDocs("comp".into()),
            E::MissingTaskDocs("run".into()),
            E::MissingTaskArgumentDocs("run".into(), "value".into()),
        ];
        let codes = errors
            .iter()
            .map(ErrorCode::formattable)
            .collect::<Vec<_>>();
        assert_eq!(codes, ["LI-00001", "LI-00002", "LI-00003"]);
    }
}
