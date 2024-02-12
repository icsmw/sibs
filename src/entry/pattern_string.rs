use crate::{
    entry::{Component, Function, VariableName},
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, Reader, Reading, E},
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
}

impl fmt::Display for Injection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::VariableName(_, v) => v.to_string(),
                Self::Function(_, v) => v.to_string(),
            },
        )
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
pub struct PatternString {
    pub pattern: String,
    pub injections: Vec<Injection>,
    pub token: usize,
}

impl Reading<PatternString> for PatternString {
    fn read(reader: &mut Reader) -> Result<Option<PatternString>, E> {
        let close = reader.open_token();
        if let Some(inner) = reader.group().closed(&chars::QUOTES) {
            let mut token = reader.token()?;
            Ok(Some(PatternString::new(
                inner,
                &mut token.bound,
                close(reader),
            )?))
        } else {
            Ok(None)
        }
    }
}

impl PatternString {
    pub fn new(pattern: String, reader: &mut Reader, token: usize) -> Result<Self, E> {
        let mut injections: Vec<Injection> = vec![];
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
        Ok(PatternString {
            pattern,
            injections,
            token,
        })
    }
}

impl fmt::Display for PatternString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.pattern,)
    }
}

impl Operator for PatternString {
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
        entry::PatternString,
        inf::tests,
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader =
            Reader::new(include_str!("../tests/reading/value_string.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = PatternString::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&entity.to_string()),
            );
            count += 1;
        }
        assert_eq!(count, 16);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), E> {
        let mut reader =
            Reader::new(include_str!("../tests/reading/value_string.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = PatternString::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&entity.to_string()),
                reader.get_fragment(&entity.token)?.content
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
        entry::{
            function::Function,
            pattern_string::{Injection, PatternString},
            variable_name::VariableName,
        },
        inf::tests::*,
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

    impl Arbitrary for PatternString {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::PatternString);
            let boxed = (
                prop::collection::vec(Injection::arbitrary_with(scope.clone()), 1..=10),
                prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 10),
            )
                .prop_map(|(injections, noise)| {
                    let mut pattern: String = String::new();
                    for (i, injection) in injections.iter().enumerate() {
                        pattern = format!("{}{{{}}}", noise[i], injection.hook());
                    }
                    PatternString {
                        injections,
                        pattern,
                        token: 0,
                    }
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::PatternString);
            boxed
        }
    }

    impl PatternString {
        pub fn arbitrary_primitive(scope: SharedScope) -> BoxedStrategy<Self> {
            scope.write().unwrap().include(Entity::PatternString);
            let boxed = "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|pattern| PatternString {
                    injections: vec![],
                    pattern,
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::PatternString);
            boxed
        }
    }
}
