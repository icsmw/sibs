use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{
        operator, term, AnyValue, Context, Formation, FormationCursor, Operator,
        OperatorPinnedResult,
    },
    reader::{chars, Reader, Reading, E},
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

impl Reading<VariableType> for VariableType {
    fn read(reader: &mut Reader) -> Result<Option<VariableType>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader.move_to().char(&[&chars::TYPE_OPEN]).is_some() {
            if let Some((word, _char)) = reader.until().char(&[&chars::TYPE_CLOSE]) {
                reader.move_to().next();
                Ok(Some(VariableType::new(word, close(reader))?))
            } else {
                Err(E::NotClosedTypeDeclaration.by_reader(reader))
            }
        } else {
            Ok(None)
        }
    }
}

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

impl Operator for VariableType {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        args: &'a [String],
        _cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let value = if args.len() != 1 {
                Err(operator::E::InvalidNumberOfArgumentsForDeclaration)?
            } else {
                args[0].to_owned()
            };
            Ok(Some(match &self.var_type {
                Types::String => AnyValue::new(value),
                Types::Number => AnyValue::new(value.parse::<isize>().map_err(|e| {
                    operator::E::ParseStringError(Types::Number.to_string(), e.to_string())
                })?),
                Types::Bool => AnyValue::new(value.parse::<bool>().map_err(|e| {
                    operator::E::ParseStringError(Types::Bool.to_string(), e.to_string())
                })?),
            }))
        })
    }
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.var_type)
    }
}

impl term::Display for VariableType {
    fn to_string(&self) -> String {
        format!("{{{}}}", self.var_type)
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
