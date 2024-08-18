use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Execute, ExecutePinnedResult, Formation, FormationCursor,
        Scope, TokenGetter, TryExecute,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Types {
    String,
    Number,
    Bool,
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String => "string",
                Self::Number => "number",
                Self::Bool => "bool",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct VariableType {
    pub var_type: Types,
    pub token: usize,
}

impl TryDissect<VariableType> for VariableType {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableType>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::VariableType);
        if reader.move_to().char(&[&chars::OPEN_CURLY_BRACE]).is_none() {
            return Ok(None);
        }
        if let Some((word, _char)) = reader.until().char(&[&chars::CLOSE_CURLY_BRACE]) {
            reader.move_to().next();
            Ok(Some(VariableType::new(word, close(reader))?))
        } else {
            Err(E::NotClosedTypeDeclaration.by_reader(reader))
        }
    }
}

impl Dissect<VariableType, VariableType> for VariableType {}

impl VariableType {
    pub fn new(var_type: String, token: usize) -> Result<Self, LinkedErr<E>> {
        if Types::String.to_string() == var_type {
            return Ok(Self {
                var_type: Types::String,
                token,
            });
        }
        if Types::Bool.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Bool,
                token,
            });
        }
        if Types::Number.to_string() == var_type {
            return Ok(Self {
                var_type: Types::Number,
                token,
            });
        }
        Err(E::UnknownVariableType(var_type).linked(&token))
    }
}

impl TokenGetter for VariableType {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExecute for VariableType {
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        args: &'a [AnyValue],
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let value = if args.len() != 1 {
                Err(operator::E::InvalidNumberOfArgumentsForDeclaration)?
            } else {
                args[0].to_owned()
            };
            Ok(Some(match &self.var_type {
                Types::String => {
                    AnyValue::String(value.as_string().ok_or(operator::E::ParseStringError(
                        Types::String.to_string(),
                        "Value isn't String".to_string(),
                    ))?)
                }
                Types::Number => {
                    AnyValue::isize(value.as_num().ok_or(operator::E::ParseStringError(
                        Types::Number.to_string(),
                        "Value isn't number".to_string(),
                    ))?)
                }
                Types::Bool => {
                    AnyValue::bool(value.as_bool().ok_or(operator::E::ParseStringError(
                        Types::Bool.to_string(),
                        "Value isn't bool".to_string(),
                    ))?)
                }
            }))
        })
    }
}

impl Execute for VariableType {}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.var_type)
    }
}

impl Formation for VariableType {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod proptest {
    use crate::elements::{Types, VariableType};
    use proptest::prelude::*;

    impl Arbitrary for Types {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![Just(Types::String), Just(Types::Bool), Just(Types::Number),].boxed()
        }
    }

    impl Arbitrary for VariableType {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            Types::arbitrary()
                .prop_map(|var_type| VariableType { var_type, token: 0 })
                .boxed()
        }
    }
}
