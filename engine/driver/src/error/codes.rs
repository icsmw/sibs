use crate::*;
use diagnostics::*;

impl ErrorCode for E {
    fn code(&self) -> &'static str {
        match self {
            Self::IO(..) => "00001",
            Self::FailExtractAnchorNodeFrom(..) => "00002",
            Self::ScriptAlreadyExecuted => "00003",
            Self::TaskInsideFuncDeclaration(..) => "00004",
            Self::NestedTasks(..) => "00005",
            Self::Parser(err) => err.code(),
            Self::Lexer(..) => "00006",
            Self::Semantic(err) => err.code(),
            Self::Runtime(err) => err.code(),
            Self::ContextError => "00007",
            Self::NotExecutable => "00008",
            Self::UsedContext => "00009",
            Self::Diagnostics(..) => "00010",
        }
    }
    fn src(&self) -> ErrorSource {
        match self {
            Self::IO(..)
            | Self::Diagnostics(..)
            | Self::FailExtractAnchorNodeFrom(..)
            | Self::ScriptAlreadyExecuted
            | Self::TaskInsideFuncDeclaration(..)
            | Self::NestedTasks(..)
            | Self::ContextError
            | Self::UsedContext
            | Self::NotExecutable
            | Self::Lexer(..) => ErrorSource::Driver,
            Self::Parser(err) => err.src(),
            Self::Semantic(err) => err.src(),
            Self::Runtime(err) => err.src(),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::*;

    impl From<&EId> for E {
        fn from(value: &EId) -> Self {
            match value {
                EId::ContextError => E::ContextError,
                EId::UsedContext => E::UsedContext,
                EId::NotExecutable => E::NotExecutable,
                EId::IO => E::IO(String::new()),
                EId::Diagnostics => E::Diagnostics(DiagnosticsError::NotFound(Uuid::new_v4())),
                EId::FailExtractAnchorNodeFrom => E::FailExtractAnchorNodeFrom(String::new()),
                EId::ScriptAlreadyExecuted => E::ScriptAlreadyExecuted,
                EId::TaskInsideFuncDeclaration => E::TaskInsideFuncDeclaration(Uuid::new_v4()),
                EId::NestedTasks => E::NestedTasks(Uuid::new_v4()),
                EId::Parser => E::Parser(ParserError::KeywordUsing),
                EId::Lexer => E::Lexer(LexerError::InvalidNumber),
                EId::Semantic => E::Semantic(SemanticError::EmptyTypeDeclaration),
                EId::Runtime => E::Runtime(runtime::RtError::NoCurrentScope),
            }
        }
    }
}
