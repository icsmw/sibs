use crate::*;
use diagnostics::*;

impl ErrorCode for E {
    fn code(&self) -> &'static str {
        match self {
            Self::NodesAreInConflict(..) => "00001",
            Self::NoClosing(..) => "00002",
            Self::UnexpectedLogicalOperator(..) => "00003",
            Self::UnexpectedBinaryOperator(..) => "00004",
            Self::MissedLogicalOperator => "00005",
            Self::MissedConditionArgument => "00006",
            Self::MissedBinaryOperator => "00007",
            Self::MissedBinaryArgument => "00008",
            Self::MissedComma => "00009",
            Self::MissedClosingBar => "00010",
            Self::MissedSemicolon => "00011",
            Self::InfiniteNumber => "00012",
            Self::NotBreakableLoop => "00013",
            Self::InvalidAssignation(..) => "00014",
            Self::MissedErrorMessage => "00015",
            Self::UnrecognizedCode(..) => "00016",
            Self::InvalidString(..) => "00017",
            Self::EmptyStringExpression => "00018",
            Self::NotSupportedStringInjection(..) => "00019",
            Self::NoExpectedBlockAfter(..) => "00020",
            Self::MissedExpectation(..) => "00021",
            Self::MissedBlock => "00022",
            Self::FailFindNode(..) => "00023",
            Self::UnexpectedEmptyParser => "00024",
            Self::FileNotFound(..) => "00025",
            Self::UnexpectedType(..) => "00026",
            Self::NoParentPath => "00027",
            Self::FileReading(..) => "00028",
            Self::FailToFindNode(..) => "00029",
            Self::IOError(..) => "00030",
            Self::FailGetModuleName(..) => "00031",
            Self::PoisonError => "00032",
            Self::BorrowMutError => "00033",
            Self::BorrowError => "00034",

            Self::MissedCallExpression => "00035",

            Self::MissedElementDeclarationInEach => "00036",
            Self::MissedIndexDeclarationInEach => "00037",
            Self::FailRecognizeElementsInEach(..) => "00038",

            Self::MissedElementDeclarationInFor => "00039",
            Self::MissedIndexDeclarationInFor => "00040",
            Self::FailRecognizeElementsInFor(..) => "00041",
            Self::InvalidForSyntax => "00042",
            Self::MissedInKeywordInFor => "00043",

            Self::MissedComparisonInWhile => "00044",

            Self::MissedActionInOptional => "00045",

            Self::MissedVariableDefinition => "00046",
            Self::MissedVariableName => "00047",
            Self::MissedVariableTypeDefinition => "00048",

            Self::KeywordUsing => "00049",

            Self::MissedNestedTypeDefinition => "00050",
            Self::UnknownType(..) => "00051",

            Self::MissedArgumentTypeDefinition => "00052",

            Self::MissedClosureBlock => "00053",
            Self::MissedClosureReturnType => "00054",

            Self::MissedFnName => "00055",
            Self::MissedFnBlock => "00056",
            Self::MissedFnArguments => "00057",

            Self::MissedModulePath => "00058",
            Self::MissedModuleBody => "00059",

            Self::InvalidPrivateKeyUsage => "00060",
            Self::MissedTaskName => "00061",
            Self::MissedTaskBlock => "00062",
            Self::MissedTaskArguments => "00063",

            Self::MissedComponentName => "00064",
            Self::MissedComponentBlock => "00065",
            Self::MissedComponentCWD => "00066",
            Self::NoTasksInComponent => "00067",

            Self::NoGatekeeperDirective => "00068",

            Self::NoSkipDirectiveArgs => "00069",
            Self::NoSkipDirectiveTaskArgs => "00070",
            Self::NoSkipDirectiveFuncCall => "00071",

            Self::InvalidReturnValue => "00072",

            Self::LexerError(..) => "00073",

            Self::Unlinked => "00074",
        }
    }
    fn src(&self) -> ErrorSource {
        ErrorSource::Parser
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::*;

    impl From<&EId> for E {
        fn from(value: &EId) -> Self {
            match value {
                EId::NodesAreInConflict => E::NodesAreInConflict(String::new()),
                EId::NoClosing => E::NoClosing(KindId::BOF),
                EId::UnexpectedLogicalOperator => E::UnexpectedLogicalOperator(KindId::BOF),
                EId::UnexpectedBinaryOperator => E::UnexpectedBinaryOperator(KindId::BOF),
                EId::MissedLogicalOperator => E::MissedLogicalOperator,
                EId::MissedConditionArgument => E::MissedConditionArgument,
                EId::MissedBinaryOperator => E::MissedBinaryOperator,
                EId::MissedBinaryArgument => E::MissedBinaryArgument,
                EId::MissedComma => E::MissedComma,
                EId::MissedClosingBar => E::MissedClosingBar,
                EId::MissedSemicolon => E::MissedSemicolon,
                EId::InfiniteNumber => E::InfiniteNumber,
                EId::NotBreakableLoop => E::NotBreakableLoop,
                EId::InvalidAssignation => E::InvalidAssignation(String::new()),
                EId::MissedErrorMessage => E::MissedErrorMessage,
                EId::UnrecognizedCode => E::UnrecognizedCode(String::new()),
                EId::InvalidString => E::InvalidString(String::new()),
                EId::EmptyStringExpression => E::EmptyStringExpression,
                EId::NotSupportedStringInjection => E::NotSupportedStringInjection(String::new()),
                EId::NoExpectedBlockAfter => E::NoExpectedBlockAfter(KindId::BOF),
                EId::MissedExpectation => E::MissedExpectation(String::new(), String::new()),
                EId::MissedBlock => E::MissedBlock,
                EId::FailFindNode => E::FailFindNode(Uuid::new_v4()),
                EId::UnexpectedEmptyParser => E::UnexpectedEmptyParser,
                EId::FileNotFound => E::FileNotFound(String::new()),
                EId::UnexpectedType => E::UnexpectedType(String::new(), String::new()),
                EId::NoParentPath => E::NoParentPath,
                EId::FileReading => E::FileReading(io::Error::other("error")),
                EId::FailToFindNode => E::FailToFindNode(String::new()),
                EId::IOError => E::IOError(io::Error::other("error")),
                EId::FailGetModuleName => E::FailGetModuleName(String::new()),
                EId::PoisonError => E::PoisonError,
                EId::BorrowMutError => E::BorrowMutError,
                EId::BorrowError => E::BorrowError,

                EId::MissedCallExpression => E::MissedCallExpression,

                EId::MissedElementDeclarationInEach => E::MissedElementDeclarationInEach,
                EId::MissedIndexDeclarationInEach => E::MissedIndexDeclarationInEach,
                EId::FailRecognizeElementsInEach => E::FailRecognizeElementsInEach(String::new()),

                EId::MissedElementDeclarationInFor => E::MissedElementDeclarationInFor,
                EId::MissedIndexDeclarationInFor => E::MissedIndexDeclarationInFor,
                EId::FailRecognizeElementsInFor => E::FailRecognizeElementsInFor(String::new()),
                EId::InvalidForSyntax => E::InvalidForSyntax,
                EId::MissedInKeywordInFor => E::MissedInKeywordInFor,

                EId::MissedComparisonInWhile => E::MissedComparisonInWhile,

                EId::MissedActionInOptional => E::MissedActionInOptional,

                EId::MissedVariableDefinition => E::MissedVariableDefinition,
                EId::MissedVariableName => E::MissedVariableName,
                EId::MissedVariableTypeDefinition => E::MissedVariableTypeDefinition,

                EId::KeywordUsing => E::KeywordUsing,

                EId::MissedNestedTypeDefinition => E::MissedNestedTypeDefinition,
                EId::UnknownType => E::UnknownType(String::new()),

                EId::MissedArgumentTypeDefinition => E::MissedArgumentTypeDefinition,

                EId::MissedClosureBlock => E::MissedClosureBlock,
                EId::MissedClosureReturnType => E::MissedClosureReturnType,

                EId::MissedFnName => E::MissedFnName,
                EId::MissedFnBlock => E::MissedFnBlock,
                EId::MissedFnArguments => E::MissedFnArguments,

                EId::MissedModulePath => E::MissedModulePath,
                EId::MissedModuleBody => E::MissedModuleBody,

                EId::InvalidPrivateKeyUsage => E::InvalidPrivateKeyUsage,
                EId::MissedTaskName => E::MissedTaskName,
                EId::MissedTaskBlock => E::MissedTaskBlock,
                EId::MissedTaskArguments => E::MissedTaskArguments,

                EId::MissedComponentName => E::MissedComponentName,
                EId::MissedComponentBlock => E::MissedComponentBlock,
                EId::MissedComponentCWD => E::MissedComponentCWD,
                EId::NoTasksInComponent => E::NoTasksInComponent,

                EId::NoGatekeeperDirective => E::NoGatekeeperDirective,

                EId::NoSkipDirectiveArgs => E::NoSkipDirectiveArgs,
                EId::NoSkipDirectiveTaskArgs => E::NoSkipDirectiveTaskArgs,
                EId::NoSkipDirectiveFuncCall => E::NoSkipDirectiveFuncCall,

                EId::InvalidReturnValue => E::InvalidReturnValue,

                EId::LexerError => E::LexerError(LexerError::InvalidNumber),
                EId::Unlinked => E::Unlinked
            }
        }
    }

    #[test]
    fn unique_codes() {
        // Make sure - no duplicates
        let codes: HashSet<_> = EId::as_vec()
            .into_iter()
            .map(|err| {
                let err: E = (&err).into();
                err.code()
            })
            .collect();
        // Make sure: order is correct and format
        assert_eq!(codes.len(), EId::as_vec().len());
        for (idx, eid) in EId::as_vec().into_iter().enumerate() {
            let err: E = (&eid).into();
            let expected = format!("{:05}", idx + 1); // "00001"
            assert_eq!(err.code(), expected, "Mismatch for {eid:?}");
        }
    }
}
