use crate::{
    elements::{Element, Metadata},
    inf::{Formation, FormationCursor},
    reader::chars,
};

impl Formation for Element {
    fn elements_count(&self) -> usize {
        match self {
            Self::Call(v, _) => v.elements_count(),
            Self::Accessor(v, _) => v.elements_count(),
            Self::Function(v, _) => v.elements_count(),
            Self::If(v, _) => v.elements_count(),
            Self::IfCondition(v, _) => v.elements_count(),
            Self::IfSubsequence(v, _) => v.elements_count(),
            Self::IfThread(v, _) => v.elements_count(),
            Self::Breaker(v, _) => v.elements_count(),
            Self::Each(v, _) => v.elements_count(),
            Self::First(v, _) => v.elements_count(),
            Self::Join(v, _) => v.elements_count(),
            Self::VariableAssignation(v, _) => v.elements_count(),
            Self::Comparing(v, _) => v.elements_count(),
            Self::Combination(v, _) => v.elements_count(),
            Self::Condition(v, _) => v.elements_count(),
            Self::Subsequence(v, _) => v.elements_count(),
            Self::Optional(v, _) => v.elements_count(),
            Self::Gatekeeper(v, _) => v.elements_count(),
            Self::Reference(v, _) => v.elements_count(),
            Self::PatternString(v, _) => v.elements_count(),
            Self::VariableName(v, _) => v.elements_count(),
            Self::Values(v, _) => v.elements_count(),
            Self::Block(v, _) => v.elements_count(),
            Self::Command(v, _) => v.elements_count(),
            Self::Task(v, _) => v.elements_count(),
            Self::Component(v, _) => v.elements_count(),
            Self::Boolean(v, _) => v.elements_count(),
            Self::Integer(v, _) => v.elements_count(),
            Self::VariableDeclaration(v, _) => v.elements_count(),
            Self::VariableVariants(v, _) => v.elements_count(),
            Self::VariableType(v, _) => v.elements_count(),
            Self::SimpleString(v, _) => v.elements_count(),
            Self::Range(v, _) => v.elements_count(),
            Self::For(v, _) => v.elements_count(),
            Self::Compute(v, _) => v.elements_count(),
            Self::Return(v, _) => v.elements_count(),
            Self::Error(v, _) => v.elements_count(),
            Self::Incrementer(v, _) => v.elements_count(),
            Self::Loop(v, _) => v.elements_count(),
            Self::While(v, _) => v.elements_count(),
            Self::Closure(v, _) => v.elements_count(),
            Self::Conclusion(v, _) => v.elements_count(),
            Self::Meta(v) => v.elements_count(),
            Self::Comment(v) => v.elements_count(),
        }
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        fn format_el<A>(el: &A, md: &Metadata, cursor: &mut FormationCursor) -> String
        where
            A: Formation,
        {
            format!(
                "{}{}{}{}{}",
                md.format(cursor),
                if md.inverting {
                    chars::EXCLAMATION.to_string()
                } else {
                    String::new()
                },
                el.format(cursor),
                if md.tolerance {
                    chars::QUESTION.to_string()
                } else {
                    String::new()
                },
                if let Some(call) = md.ppm.as_ref() {
                    call.to_string()
                } else {
                    String::new()
                }
            )
        }
        match self {
            Self::Call(v, m) => format_el(v, m, cursor),
            Self::Accessor(v, m) => format_el(v, m, cursor),
            Self::Function(v, m) => format_el(v, m, cursor),
            Self::If(v, m) => format_el(v, m, cursor),
            Self::IfCondition(v, m) => format_el(v, m, cursor),
            Self::IfSubsequence(v, m) => format_el(v, m, cursor),
            Self::IfThread(v, m) => format_el(v, m, cursor),
            Self::Breaker(v, m) => format_el(v, m, cursor),
            Self::Each(v, m) => format_el(v, m, cursor),
            Self::First(v, m) => format_el(v, m, cursor),
            Self::Join(v, m) => format_el(v, m, cursor),
            Self::VariableAssignation(v, m) => format_el(v, m, cursor),
            Self::Comparing(v, m) => format_el(v, m, cursor),
            Self::Combination(v, m) => format_el(v, m, cursor),
            Self::Condition(v, m) => format_el(v, m, cursor),
            Self::Subsequence(v, m) => format_el(v, m, cursor),
            Self::Optional(v, m) => format_el(v, m, cursor),
            Self::Gatekeeper(v, m) => format_el(v, m, cursor),
            Self::Reference(v, m) => format_el(v, m, cursor),
            Self::PatternString(v, m) => format_el(v, m, cursor),
            Self::VariableName(v, m) => format_el(v, m, cursor),
            Self::Values(v, m) => format_el(v, m, cursor),
            Self::Block(v, m) => format_el(v, m, cursor),
            Self::Command(v, m) => format_el(v, m, cursor),
            Self::Task(v, m) => format_el(v, m, cursor),
            Self::Component(v, m) => format_el(v, m, cursor),
            Self::Boolean(v, m) => format_el(v, m, cursor),
            Self::Integer(v, m) => format_el(v, m, cursor),
            Self::VariableDeclaration(v, m) => format_el(v, m, cursor),
            Self::VariableVariants(v, m) => format_el(v, m, cursor),
            Self::VariableType(v, m) => format_el(v, m, cursor),
            Self::SimpleString(v, m) => format_el(v, m, cursor),
            Self::Range(v, m) => format_el(v, m, cursor),
            Self::For(v, m) => format_el(v, m, cursor),
            Self::Compute(v, m) => format_el(v, m, cursor),
            Self::Return(v, m) => format_el(v, m, cursor),
            Self::Error(v, m) => format_el(v, m, cursor),
            Self::Incrementer(v, m) => format_el(v, m, cursor),
            Self::Loop(v, m) => format_el(v, m, cursor),
            Self::While(v, m) => format_el(v, m, cursor),
            Self::Closure(v, m) => format_el(v, m, cursor),
            Self::Conclusion(v, m) => format_el(v, m, cursor),
            Self::Meta(v) => v.format(cursor),
            Self::Comment(v) => v.format(cursor),
        }
    }
}
