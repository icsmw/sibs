use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Component, Function, Reader, Reading, VariableName},
        E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Injection {
    VariableName(String, VariableName),
    Function(String, Function),
}

impl Injection {
    pub fn hook(&self) -> &str {
        match self {
            Self::VariableName(hook, _) => hook,
            Self::Function(hook, _) => hook,
        }
    }
    pub fn token(&self) -> usize {
        match self {
            Self::VariableName(_, v) => v.token,
            Self::Function(_, v) => v.token,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::VariableName(_, v) => v.to_string(),
            Self::Function(_, v) => v.to_string(),
        }
    }
}

impl Operator for Injection {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::VariableName(_, v) => v.process(owner, components, args, cx).await,
                Self::Function(_, v) => v.process(owner, components, args, cx).await,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct ValueString {
    pub pattern: String,
    pub injections: Vec<Injection>,
    pub token: usize,
}

impl Reading<ValueString> for ValueString {
    fn read(reader: &mut Reader) -> Result<Option<ValueString>, E> {
        if let Some(inner) = reader.group().closed(&chars::QUOTES) {
            let mut token = reader.token()?;
            Ok(Some(ValueString::new(inner, &mut token.bound)?))
        } else {
            Ok(None)
        }
    }
}

impl ValueString {
    pub fn new(pattern: String, reader: &mut Reader) -> Result<Self, E> {
        let mut injections: Vec<Injection> = vec![];
        let token = reader.token()?.id;
        while reader.seek_to().char(&chars::TYPE_OPEN) {
            reader.move_to().next();
            if reader.until().char(&[&chars::TYPE_CLOSE]).is_some() {
                let mut token = reader.token()?;
                let hook = token.content.clone();
                reader.move_to().next();
                if let Some(variable_name) = VariableName::read(&mut token.bound)? {
                    injections.push(Injection::VariableName(hook, variable_name));
                } else if let Some(func) = Function::read(&mut token.bound)? {
                    injections.push(Injection::Function(hook, func));
                } else {
                    Err(E::NoVariableReference)?
                }
            } else {
                Err(E::NoInjectionClose)?
            }
        }
        Ok(ValueString {
            pattern,
            injections,
            token,
        })
    }
}

impl fmt::Display for ValueString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.pattern,)
    }
}

impl Operator for ValueString {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let mut output = self.pattern.clone();
            for injection in self.injections.iter() {
                let val = injection
                    .process(owner, components, args, cx)
                    .await?
                    .ok_or(operator::E::FailToExtractValue)?
                    .get_as_string()
                    .ok_or(operator::E::FailToGetValueAsString)?;
                let hook = format!("{{{}}}", injection.hook());
                output = output.replace(&hook, &val);
            }
            Ok(Some(AnyValue::new(output)))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        inf::tests,
        reader::{
            entry::{Reading, ValueString},
            Reader, E,
        },
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader =
            Reader::new(include_str!("../../tests/reading/value_string.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = ValueString::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string()),
            );
            assert_eq!(
                tests::trim(&entity.to_string()),
                format!("\"{}\"", reader.get_fragment(&entity.token)?.content)
            );
            for injection in entity.injections.iter() {
                assert_eq!(
                    injection.to_string(),
                    reader.get_fragment(&injection.token())?.content
                );
            }
            count += 1;
        }
        assert_eq!(count, 16);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        inf::tests::*,
        reader::entry::{
            function::Function,
            value_strings::{Injection, ValueString},
            variable_name::VariableName,
        },
    };
    use proptest::prelude::*;

    impl Arbitrary for Injection {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            let permissions = scope.read().unwrap().permissions();
            let mut allowed = vec![VariableName::arbitrary()
                .prop_map(|v| Injection::VariableName(v.to_string(), v))
                .boxed()];
            if permissions.func {
                allowed.push(
                    Function::arbitrary_with(scope.clone())
                        .prop_map(|f| Injection::Function(f.to_string(), f))
                        .boxed(),
                );
            }
            prop::strategy::Union::new(allowed).boxed()
        }
    }

    impl Arbitrary for ValueString {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::ValueString);
            let boxed = (
                prop::collection::vec(Injection::arbitrary_with(scope.clone()), 1..=10),
                prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 10),
            )
                .prop_map(|(injections, noise)| {
                    let mut pattern: String = String::new();
                    for (i, injection) in injections.iter().enumerate() {
                        pattern = format!("{}{{{}}}", noise[i], injection.hook());
                    }
                    ValueString {
                        injections,
                        pattern,
                        token: 0,
                    }
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::ValueString);
            boxed
        }
    }

    impl ValueString {
        pub fn arbitrary_primitive(scope: SharedScope) -> BoxedStrategy<Self> {
            scope.write().unwrap().include(Entity::ValueString);
            let boxed = "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|pattern| ValueString {
                    injections: vec![],
                    pattern,
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::ValueString);
            boxed
        }
    }
}
