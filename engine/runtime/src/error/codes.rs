use crate::*;
use diagnostics::*;

impl ErrorCode for E {
    fn code(&self) -> &'static str {
        match self {
            Self::AttemptToLeaveGlobalContext => "00001",
            Self::NoCurrentScope => "00002",
            Self::ScopeNotFound(..) => "00003",
            Self::RecvError => "00004",
            Self::SendError => "00005",
            Self::FailExtractValue => "00006",
            Self::FailGetSrcLink => "00007",
            Self::InvalidValueType(..) => "00008",
            Self::NotPublicValueType => "00009",
            Self::CannotBeConvertedToString => "00010",
            Self::MissedBinaryOperator => "00011",
            Self::NotComparableValue => "00012",
            Self::DifferentTypeOfValues => "00013",
            Self::InvalidComparisonSeq => "00014",
            Self::InvalidIfStatement => "00015",
            Self::FailInferType => "00016",
            Self::UnexpectedNode(..) => "00017",
            Self::UndefinedVariable(..) => "00018",
            Self::AttemptToLeaveRootContextLevel => "00019",
            Self::NoCurrentContextLevel => "00020",
            Self::NoRootContext => "00021",
            Self::FailToFindContext(..) => "00022",
            Self::FailCovertToRsType(..) => "00023",
            Self::VariableNotFound(..) => "00024",
            Self::NotApplicableToTypeOperation => "00025",
            Self::InvalidType(..) => "00026",
            Self::DismatchValueType(..) => "00027",

            Self::FuncAlreadyRegistered(..) => "00028",
            Self::ClosureAlreadyRegistered(..) => "00029",
            Self::FuncNotFound(..) => "00030",
            Self::ClosureNotFound(..) => "00031",
            Self::InvalidFnArgument => "00032",
            Self::InvalidFnArgumentsNumber(..) => "00033",
            Self::MissedFnArgument(..) => "00034",
            Self::InvalidFnArgumentType => "00035",
            Self::NoParentValueToCallFn => "00036",
            Self::FnArgumentTypeDismatch(..) => "00037",
            Self::NoLinkedFunctions(..) => "00038",
            Self::NotInitedFunction(..) => "00039",
            Self::NotInitedClosure(..) => "00040",

            Self::CompNotFound(..) => "00041",
            Self::TaskNotFound(..) => "00042",
            Self::NotInitedTask(..) => "00043",
            Self::InvalidTaskArgument => "00044",
            Self::InvalidTaskArgumentType => "00045",
            Self::TaskArgumentTypeDismatch(..) => "00046",
            Self::NoLinkedTaskCallers(..) => "00047",

            Self::MultipleRepeatedFnArgsDeclared => "00048",
            Self::NotLastRepeatedFnArg => "00049",

            Self::IO(..) => "00050",
            Self::SysTime(..) => "00051",
            Self::Storage(..) => "00052",

            Self::FnUsesKeyword(..) => "00053",

            Self::TaskDuplicate => "00054",
            Self::NoMasterComponent(..) => "00055",

            Self::InvalidIterationSource => "00056",

            Self::NoBreakSignalFor(..) => "00057",
            Self::BreakSignalAlreadyExist(..) => "00058",
            Self::LoopAlreadyExist(..) => "00059",
            Self::NoOpenLoopsToBreak => "00060",
            Self::NoOpenLoopsToClose => "00061",

            Self::ReturnCXAlreadyExist(..) => "00062",
            Self::NoOpenReturnCXToBreak => "00063",
            Self::NoOpenReturnCXsToClose => "00064",
            Self::ReturnValueAlreadyExist(..) => "00065",

            Self::RenderTemplateErr(..) => "00066",
            Self::NoProgressForTask(..) => "00067",

            Self::SpawnSetup(..) => "00068",
            Self::SpawnError(..) => "00069",
            Self::SpawnFailed(..) => "00070",

            Self::Timestamp => "00071",

            Self::JobAlreadyExists(..) => "00072",
            Self::JobDoesNotExist(..) => "00073",

            Self::JoinError(..) => "00074",
            Self::FailToFindJoinResult(..) => "00075",
            Self::SomeNodesHadSameUuid => "00076",

            Self::MultipleSignalEmit(..) => "00077",

            Self::Other(..) => "00078",

            Self::Journal(..) => "00079",
        }
    }
    fn src(&self) -> ErrorSource {
        ErrorSource::Runtime
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, io};

    use crate::*;

    impl From<&EId> for E {
        fn from(value: &EId) -> Self {
            match value {
                EId::AttemptToLeaveGlobalContext => E::AttemptToLeaveGlobalContext,
                EId::NoCurrentScope => E::NoCurrentScope,
                EId::ScopeNotFound => E::ScopeNotFound(Uuid::new_v4()),
                EId::RecvError => E::RecvError,
                EId::SendError => E::SendError,
                EId::FailExtractValue => E::FailExtractValue,
                EId::FailGetSrcLink => E::FailGetSrcLink,
                EId::InvalidValueType => E::InvalidValueType(String::new()),
                EId::NotPublicValueType => E::NotPublicValueType,
                EId::CannotBeConvertedToString => E::CannotBeConvertedToString,
                EId::MissedBinaryOperator => E::MissedBinaryOperator,
                EId::NotComparableValue => E::NotComparableValue,
                EId::DifferentTypeOfValues => E::DifferentTypeOfValues,
                EId::InvalidComparisonSeq => E::InvalidComparisonSeq,
                EId::InvalidIfStatement => E::InvalidIfStatement,
                EId::FailInferType => E::FailInferType,
                EId::UnexpectedNode => E::UnexpectedNode(NodeId::Root),
                EId::UndefinedVariable => E::UndefinedVariable(String::new()),
                EId::AttemptToLeaveRootContextLevel => E::AttemptToLeaveRootContextLevel,
                EId::NoCurrentContextLevel => E::NoCurrentContextLevel,
                EId::NoRootContext => E::NoRootContext,
                EId::FailToFindContext => E::FailToFindContext(Uuid::new_v4()),
                EId::FailCovertToRsType => E::FailCovertToRsType(String::new(), String::new()),
                EId::VariableNotFound => E::VariableNotFound(String::new()),
                EId::NotApplicableToTypeOperation => E::NotApplicableToTypeOperation,
                EId::InvalidType => E::InvalidType(Ty::Undefined, RtValue::Error),
                EId::DismatchValueType => E::DismatchValueType(String::new(), String::new()),

                EId::FuncAlreadyRegistered => E::FuncAlreadyRegistered(String::new()),
                EId::ClosureAlreadyRegistered => E::ClosureAlreadyRegistered(Uuid::new_v4()),
                EId::FuncNotFound => E::FuncNotFound(String::new()),
                EId::ClosureNotFound => E::ClosureNotFound(Uuid::new_v4()),
                EId::InvalidFnArgument => E::InvalidFnArgument,
                EId::InvalidFnArgumentsNumber => E::InvalidFnArgumentsNumber(0, 0),
                EId::MissedFnArgument => E::MissedFnArgument(String::new()),
                EId::InvalidFnArgumentType => E::InvalidFnArgumentType,
                EId::NoParentValueToCallFn => E::NoParentValueToCallFn,
                EId::FnArgumentTypeDismatch => E::FnArgumentTypeDismatch(String::new()),
                EId::NoLinkedFunctions => E::NoLinkedFunctions(Uuid::new_v4()),
                EId::NotInitedFunction => E::NotInitedFunction(String::new()),
                EId::NotInitedClosure => E::NotInitedClosure(Uuid::new_v4()),

                EId::CompNotFound => E::CompNotFound(String::new()),
                EId::TaskNotFound => E::TaskNotFound(String::new(), String::new()),
                EId::NotInitedTask => E::NotInitedTask(String::new()),
                EId::InvalidTaskArgument => E::InvalidTaskArgument,
                EId::InvalidTaskArgumentType => E::InvalidTaskArgumentType,
                EId::TaskArgumentTypeDismatch => E::TaskArgumentTypeDismatch(String::new()),
                EId::NoLinkedTaskCallers => E::NoLinkedTaskCallers(Uuid::new_v4()),

                EId::MultipleRepeatedFnArgsDeclared => E::MultipleRepeatedFnArgsDeclared,
                EId::NotLastRepeatedFnArg => E::NotLastRepeatedFnArg,

                EId::IO => E::IO(io::Error::other(String::new())),
                EId::SysTime => E::SysTime(String::new()),
                EId::Storage => E::Storage(String::new()),

                EId::FnUsesKeyword => E::FnUsesKeyword(String::new(), String::new()),

                EId::TaskDuplicate => E::TaskDuplicate,
                EId::NoMasterComponent => E::NoMasterComponent(String::new()),

                EId::InvalidIterationSource => E::InvalidIterationSource,

                EId::NoBreakSignalFor => E::NoBreakSignalFor(Uuid::new_v4()),
                EId::BreakSignalAlreadyExist => E::BreakSignalAlreadyExist(Uuid::new_v4()),
                EId::LoopAlreadyExist => E::LoopAlreadyExist(Uuid::new_v4()),
                EId::NoOpenLoopsToBreak => E::NoOpenLoopsToBreak,
                EId::NoOpenLoopsToClose => E::NoOpenLoopsToClose,

                EId::ReturnCXAlreadyExist => E::ReturnCXAlreadyExist(Uuid::new_v4()),
                EId::NoOpenReturnCXToBreak => E::NoOpenReturnCXToBreak,
                EId::NoOpenReturnCXsToClose => E::NoOpenReturnCXsToClose,
                EId::ReturnValueAlreadyExist => E::ReturnValueAlreadyExist(Uuid::new_v4()),

                EId::RenderTemplateErr => E::RenderTemplateErr(String::new()),
                EId::NoProgressForTask => E::NoProgressForTask(Uuid::new_v4()),

                EId::SpawnSetup => E::SpawnSetup(String::new(), String::new()),
                EId::SpawnError => E::SpawnError(String::new(), String::new()),
                EId::SpawnFailed => E::SpawnFailed(String::new()),

                EId::Timestamp => E::Timestamp,

                EId::JobAlreadyExists => E::JobAlreadyExists(Uuid::new_v4(), String::new()),
                EId::JobDoesNotExist => E::JobDoesNotExist(Uuid::new_v4()),

                EId::JoinError => E::JoinError(String::new()),
                EId::FailToFindJoinResult => E::FailToFindJoinResult(Uuid::new_v4()),
                EId::SomeNodesHadSameUuid => E::SomeNodesHadSameUuid,

                EId::MultipleSignalEmit => E::MultipleSignalEmit(String::new()),

                EId::Other => E::Other(String::new()),

                EId::Journal => E::Journal(String::new()),
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
