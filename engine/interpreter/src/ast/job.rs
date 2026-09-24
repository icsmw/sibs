use crate::*;

impl NodeJobVisibility for LinkedNode {
    fn get_visibility(&self) -> JobVisibility {
        match self.get_node() {
            Node::Root(node) => match node {
                Root::Component(..) | Root::Task(..) => JobVisibility::Visible,
                Root::Anchor(..) | Root::Module(..) => JobVisibility::Hidden,
            },
            Node::Statement(node) => match node {
                Statement::Join(..) => JobVisibility::Visible,
                Statement::Block(..)
                | Statement::Break(..)
                | Statement::Return(..)
                | Statement::Optional(..)
                | Statement::If(..)
                | Statement::For(..)
                | Statement::While(..)
                | Statement::Loop(..)
                | Statement::Assignation(..)
                | Statement::AssignedValue(..)
                | Statement::ArgumentAssignation(..)
                | Statement::ArgumentAssignedValue(..)
                | Statement::OneOf(..) => JobVisibility::Hidden,
            },
            Node::Expression(node) => match node {
                Expression::Command(..) => JobVisibility::Visible,
                Expression::Call(..)
                | Expression::Accessor(..)
                | Expression::LogicalOp(..)
                | Expression::ComparisonOp(..)
                | Expression::Comparison(..)
                | Expression::ComparisonGroup(..)
                | Expression::ComparisonSeq(..)
                | Expression::Range(..)
                | Expression::Variable(..)
                | Expression::BinaryExpSeq(..)
                | Expression::BinaryExp(..)
                | Expression::BinaryExpGroup(..)
                | Expression::BinaryOp(..)
                | Expression::FunctionCall(..)
                | Expression::CompoundAssignments(..)
                | Expression::CompoundAssignmentsOp(..)
                | Expression::TaskCall(..) => JobVisibility::Hidden,
            },
            Node::ControlFlowModifier(node) => match node {
                ControlFlowModifier::Gatekeeper(..) | ControlFlowModifier::Skip(..) => {
                    JobVisibility::Hidden
                }
            },
            Node::Declaration(node) => match node {
                Declaration::IncludeDeclaration(..)
                | Declaration::ModuleDeclaration(..)
                | Declaration::FunctionDeclaration(..)
                | Declaration::VariableDeclaration(..)
                | Declaration::ArgumentDeclaration(..)
                | Declaration::VariableVariants(..)
                | Declaration::VariableType(..)
                | Declaration::VariableTypeDeclaration(..)
                | Declaration::VariableName(..)
                | Declaration::ClosureDeclaration(..) => JobVisibility::Hidden,
            },
            Node::Value(node) => match node {
                Value::Error(..)
                | Value::Boolean(..)
                | Value::Number(..)
                | Value::Array(..)
                | Value::InterpolatedString(..)
                | Value::PrimitiveString(..)
                | Value::Closure(..) => JobVisibility::Hidden,
            },
            Node::Miscellaneous(node) => match node {
                Miscellaneous::RootMeta(..)
                | Miscellaneous::Meta(..)
                | Miscellaneous::Comment(..) => JobVisibility::Hidden,
            },
        }
    }
}

impl NodeJobName for LinkedNode {
    fn get_job_name(&self) -> String {
        let node = self.get_node();
        match node {
            Node::Root(inner) => match inner {
                Root::Component(component) => component.get_name(),
                Root::Task(task) => task.get_name(),
                Root::Anchor(..) | Root::Module(..) => node.id().to_string(),
            },
            Node::Statement(inner) => match inner {
                Statement::Join(..) => "joining".to_owned(),
                Statement::Block(..)
                | Statement::Break(..)
                | Statement::Return(..)
                | Statement::Optional(..)
                | Statement::If(..)
                | Statement::For(..)
                | Statement::While(..)
                | Statement::Loop(..)
                | Statement::Assignation(..)
                | Statement::AssignedValue(..)
                | Statement::ArgumentAssignation(..)
                | Statement::ArgumentAssignedValue(..)
                | Statement::OneOf(..) => node.id().to_string(),
            },
            Node::Expression(inner) => match inner {
                Expression::Command(command) => command.to_string(),
                Expression::Call(..)
                | Expression::Accessor(..)
                | Expression::LogicalOp(..)
                | Expression::ComparisonOp(..)
                | Expression::Comparison(..)
                | Expression::ComparisonGroup(..)
                | Expression::ComparisonSeq(..)
                | Expression::Range(..)
                | Expression::Variable(..)
                | Expression::BinaryExpSeq(..)
                | Expression::BinaryExp(..)
                | Expression::BinaryExpGroup(..)
                | Expression::BinaryOp(..)
                | Expression::FunctionCall(..)
                | Expression::CompoundAssignments(..)
                | Expression::CompoundAssignmentsOp(..)
                | Expression::TaskCall(..) => node.id().to_string(),
            },
            Node::ControlFlowModifier(inner) => match inner {
                ControlFlowModifier::Gatekeeper(..) | ControlFlowModifier::Skip(..) => {
                    node.id().to_string()
                }
            },
            Node::Declaration(inner) => match inner {
                Declaration::IncludeDeclaration(..)
                | Declaration::ModuleDeclaration(..)
                | Declaration::FunctionDeclaration(..)
                | Declaration::VariableDeclaration(..)
                | Declaration::ArgumentDeclaration(..)
                | Declaration::VariableVariants(..)
                | Declaration::VariableType(..)
                | Declaration::VariableTypeDeclaration(..)
                | Declaration::VariableName(..)
                | Declaration::ClosureDeclaration(..) => node.id().to_string(),
            },
            Node::Value(inner) => match inner {
                Value::Error(..)
                | Value::Boolean(..)
                | Value::Number(..)
                | Value::Array(..)
                | Value::InterpolatedString(..)
                | Value::PrimitiveString(..)
                | Value::Closure(..) => node.id().to_string(),
            },
            Node::Miscellaneous(inner) => match inner {
                Miscellaneous::RootMeta(..)
                | Miscellaneous::Meta(..)
                | Miscellaneous::Comment(..) => node.id().to_string(),
            },
        }
    }
}
