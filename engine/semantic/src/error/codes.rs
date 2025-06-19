use crate::*;
use diagnostics::*;

impl ErrorCode for E {
    fn code(&self) -> &'static str {
        match self {
            Self::TokenIsNotBoundToKnownTy => "00001",
            Self::NoVariantsAreDefined => "00002",
            Self::VariantsHaveDiffTypes => "00003",
            Self::DismatchTypes(..) => "00004",
            Self::IndeterminateType => "00005",
            Self::NotBreakableLoop => "00006",
            Self::NotAllowedFnDeclaration => "00007",
            Self::NotAssignedBreak => "00008",
            Self::NotAssignedReturn => "00009",
            Self::MissedAssignedAndAnnotatedType => "00010",
            Self::AttemptToLeaveGlobalScope => "00011",
            Self::AttemptToLeaveRootScopeLevel => "00012",
            Self::NoCurrentScopeLevel => "00013",
            Self::NoRootScope => "00014",
            Self::FailToFindScope(..) => "00015",
            Self::InvalidIfStatement => "00016",
            Self::VariableIsNotDefined(..) => "00017",
            Self::NegationToNotBool => "00018",
            Self::UnexpectedNode(..) => "00019",
            Self::EmptyTypeDeclaration => "00020",
            Self::ExpectedBoolType(..) => "00021",
            Self::ExpectedNumericType(..) => "00022",
            Self::AccessorWithoutParent => "00023",
            Self::CallWithoutParent => "00024",
            Self::NoFnCallNodeFound => "00025",
            Self::AccessorOnWrongType(..) => "00026",
            Self::FuncExists(..) => "00027",
            Self::InvalidModuleName => "00028",
            Self::InvalidFnName => "00029",
            Self::InvalidFnArg => "00030",
            Self::FnDeclarationError(..) => "00031",
            Self::FailInferDeterminedType(..) => "00032",
            Self::FnNotFound(..) => "00033",
            Self::FnArgsNumberDismatch(..) => "00034",
            Self::FailInferFnResultType(..) => "00035",
            Self::FuncAlreadyRegistered(..) => "00036",
            Self::ClosureNotInited(..) => "00037",
            Self::MultipleRepeatedFnArgs => "00038",
            Self::InvalidTaskArg => "00039",
            Self::FailToGetMasterOfTask => "00040",
            Self::TaskNotFound(..) => "00041",
            Self::TaskArgsNumberDismatch(..) => "00042",
            Self::TypeCannotUsedInContext => "00043",
            Self::InvalidIterationSource => "00044",
            Self::RtError(err) => err.code(),
        }
    }
    fn src(&self) -> ErrorSource {
        match self {
            Self::TokenIsNotBoundToKnownTy
            | Self::NoVariantsAreDefined
            | Self::VariantsHaveDiffTypes
            | Self::DismatchTypes(..)
            | Self::IndeterminateType
            | Self::NotBreakableLoop
            | Self::NotAllowedFnDeclaration
            | Self::NotAssignedBreak
            | Self::NotAssignedReturn
            | Self::MissedAssignedAndAnnotatedType
            | Self::AttemptToLeaveGlobalScope
            | Self::AttemptToLeaveRootScopeLevel
            | Self::NoCurrentScopeLevel
            | Self::NoRootScope
            | Self::FailToFindScope(..)
            | Self::InvalidIfStatement
            | Self::VariableIsNotDefined(..)
            | Self::NegationToNotBool
            | Self::UnexpectedNode(..)
            | Self::EmptyTypeDeclaration
            | Self::ExpectedBoolType(..)
            | Self::ExpectedNumericType(..)
            | Self::AccessorWithoutParent
            | Self::CallWithoutParent
            | Self::NoFnCallNodeFound
            | Self::AccessorOnWrongType(..)
            | Self::FuncExists(..)
            | Self::InvalidModuleName
            | Self::InvalidFnName
            | Self::InvalidFnArg
            | Self::FnDeclarationError(..)
            | Self::FailInferDeterminedType(..)
            | Self::FnNotFound(..)
            | Self::FnArgsNumberDismatch(..)
            | Self::FailInferFnResultType(..)
            | Self::FuncAlreadyRegistered(..)
            | Self::ClosureNotInited(..)
            | Self::MultipleRepeatedFnArgs
            | Self::InvalidTaskArg
            | Self::FailToGetMasterOfTask
            | Self::TaskNotFound(..)
            | Self::TaskArgsNumberDismatch(..)
            | Self::TypeCannotUsedInContext
            | Self::InvalidIterationSource => ErrorSource::Semantic,
            Self::RtError(err) => err.src(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::*;

    impl From<&EId> for E {
        fn from(value: &EId) -> Self {
            match value {
                EId::TokenIsNotBoundToKnownTy => E::TokenIsNotBoundToKnownTy,
                EId::NoVariantsAreDefined => E::NoVariantsAreDefined,
                EId::VariantsHaveDiffTypes => E::VariantsHaveDiffTypes,
                EId::DismatchTypes => E::DismatchTypes(String::new()),
                EId::IndeterminateType => E::IndeterminateType,
                EId::NotBreakableLoop => E::NotBreakableLoop,
                EId::NotAllowedFnDeclaration => E::NotAllowedFnDeclaration,
                EId::NotAssignedBreak => E::NotAssignedBreak,
                EId::NotAssignedReturn => E::NotAssignedReturn,
                EId::MissedAssignedAndAnnotatedType => E::MissedAssignedAndAnnotatedType,
                EId::AttemptToLeaveGlobalScope => E::AttemptToLeaveGlobalScope,
                EId::AttemptToLeaveRootScopeLevel => E::AttemptToLeaveRootScopeLevel,
                EId::NoRootScope => E::NoRootScope,
                EId::NoCurrentScopeLevel => E::NoCurrentScopeLevel,
                EId::FailToFindScope => E::FailToFindScope(Uuid::new_v4()),
                EId::InvalidIfStatement => E::InvalidIfStatement,
                EId::VariableIsNotDefined => E::VariableIsNotDefined(String::new()),
                EId::NegationToNotBool => E::NegationToNotBool,
                EId::UnexpectedNode => E::UnexpectedNode(NodeId::Root),
                EId::EmptyTypeDeclaration => E::EmptyTypeDeclaration,
                EId::ExpectedBoolType => E::ExpectedBoolType(Ty::Undefined),
                EId::ExpectedNumericType => E::ExpectedNumericType(Ty::Undefined),
                EId::AccessorWithoutParent => E::AccessorWithoutParent,
                EId::CallWithoutParent => E::CallWithoutParent,
                EId::NoFnCallNodeFound => E::NoFnCallNodeFound,
                EId::AccessorOnWrongType => E::AccessorOnWrongType(Ty::Undefined),
                EId::FuncExists => E::FuncExists(String::new()),
                EId::InvalidModuleName => E::InvalidModuleName,
                EId::InvalidFnName => E::InvalidFnName,
                EId::InvalidFnArg => E::InvalidFnArg,
                EId::FnDeclarationError => E::FnDeclarationError(String::new()),
                EId::FailInferDeterminedType => E::FailInferDeterminedType(Ty::Undefined),
                EId::FnNotFound => E::FnNotFound(String::new()),
                EId::FnArgsNumberDismatch => E::FnArgsNumberDismatch(String::new(), 0, 0),
                EId::FailInferFnResultType => E::FailInferFnResultType(String::new()),
                EId::FuncAlreadyRegistered => E::FuncAlreadyRegistered(String::new()),
                EId::ClosureNotInited => E::ClosureNotInited(Uuid::new_v4()),
                EId::MultipleRepeatedFnArgs => E::MultipleRepeatedFnArgs,
                EId::InvalidTaskArg => E::InvalidTaskArg,
                EId::FailToGetMasterOfTask => E::FailToGetMasterOfTask,
                EId::TaskNotFound => E::TaskNotFound(String::new()),
                EId::TaskArgsNumberDismatch => E::TaskArgsNumberDismatch(String::new(), 0, 0),
                EId::TypeCannotUsedInContext => E::TypeCannotUsedInContext,
                EId::InvalidIterationSource => E::InvalidIterationSource,
                EId::RtError => E::RtError(RtError::NoCurrentScope),
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
