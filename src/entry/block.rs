use crate::{
    entry::{
        Command, Component, Each, First, Function, If, Meta, Optional, PatternString, Reference,
        VariableAssignation, VariableName,
    },
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
        term::{self, Term},
    },
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Element {
    Function(Function),
    If(If),
    Each(Each),
    First(First),
    VariableAssignation(VariableAssignation),
    Optional(Optional),
    Reference(Reference),
    PatternString(PatternString),
    VariableName(VariableName),
    Command(Command),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Command(v) => v.to_string(),
                Self::Function(v) => v.to_string(),
                Self::If(v) => v.to_string(),
                Self::Each(v) => v.to_string(),
                Self::First(v) => v.to_string(),
                Self::VariableAssignation(v) => v.to_string(),
                Self::Optional(v) => v.to_string(),
                Self::Reference(v) => v.to_string(),
                Self::PatternString(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
            }
        )
    }
}

impl Operator for Element {
    fn token(&self) -> usize {
        match self {
            Self::Command(v) => v.token(),
            Self::Function(v) => v.token(),
            Self::If(v) => v.token(),
            Self::Each(v) => v.token(),
            Self::First(v) => v.token(),
            Self::VariableAssignation(v) => v.token(),
            Self::Optional(v) => v.token(),
            Self::Reference(v) => v.token(),
            Self::PatternString(v) => v.token(),
            Self::VariableName(v) => v.token(),
        }
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::Command(v) => v.execute(owner, components, args, cx).await,
                Self::Function(v) => v.execute(owner, components, args, cx).await,
                Self::If(v) => v.execute(owner, components, args, cx).await,
                Self::Each(v) => v.execute(owner, components, args, cx).await,
                Self::First(v) => v.execute(owner, components, args, cx).await,
                Self::VariableAssignation(v) => v.execute(owner, components, args, cx).await,
                Self::Optional(v) => v.execute(owner, components, args, cx).await,
                Self::Reference(v) => v.execute(owner, components, args, cx).await,
                Self::PatternString(v) => v.execute(owner, components, args, cx).await,
                Self::VariableName(v) => v.execute(owner, components, args, cx).await,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub meta: Option<Meta>,
    pub elements: Vec<Element>,
    pub by_first: bool,
    pub token: usize,
}

impl Block {
    pub fn use_as_first(&mut self) {
        self.by_first = true;
    }
}

impl Reading<Block> for Block {
    fn read(reader: &mut Reader) -> Result<Option<Block>, E> {
        let close = reader.open_token();
        if reader
            .group()
            .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            let mut elements: Vec<Element> = vec![];
            let mut meta: Option<Meta> = None;
            while !inner.rest().trim().is_empty() {
                if let Some(md) = Meta::read(&mut inner)? {
                    meta = Some(md);
                    continue;
                }
                if let Some(el) = If::read(&mut inner)? {
                    elements.push(Element::If(el));
                    continue;
                }
                if let Some(el) = Optional::read(&mut inner)? {
                    elements.push(Element::Optional(el));
                    continue;
                }
                inner.state().set();
                if let Some(el) = VariableName::read(&mut inner)? {
                    if let Some(chars::SEMICOLON) =
                        inner.move_to().char(&[&chars::SEMICOLON, &chars::EQUAL])
                    {
                        elements.push(Element::VariableName(el));
                        continue;
                    }
                }
                inner.state().restore()?;
                if let Some(el) = VariableAssignation::read(&mut inner)? {
                    elements.push(Element::VariableAssignation(el));
                    continue;
                }
                if let Some(el) = Each::read(&mut inner)? {
                    elements.push(Element::Each(el));
                    continue;
                }
                if let Some(el) = First::read(&mut inner)? {
                    elements.push(Element::First(el));
                    continue;
                }
                if let Some(el) = Reference::read(&mut inner)? {
                    elements.push(Element::Reference(el));
                    continue;
                }
                if let Some(el) = PatternString::read(&mut inner)? {
                    if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        Err(reader.report_err(&inner.token()?.id, E::MissedSemicolon)?)?;
                    }
                    elements.push(Element::PatternString(el));
                    continue;
                }
                if let Some(el) = Function::read(&mut inner)? {
                    elements.push(Element::Function(el));
                    continue;
                }
                if let Some((cmd, _)) = inner.until().char(&[&chars::SEMICOLON]) {
                    inner.move_to().next();
                    elements.push(Element::Command(Command::new(cmd, inner.token()?.id)?));
                } else {
                    break;
                }
            }
            Ok(if elements.is_empty() {
                None
            } else {
                Some(Block {
                    elements,
                    meta,
                    token: close(reader),
                    by_first: false,
                })
            })
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[\n{}{}{}]",
            self.meta
                .as_ref()
                .map(|meta| {
                    format!(
                        "{}{}",
                        meta.inner
                            .iter()
                            .map(|v| format!("/// {v}"))
                            .collect::<Vec<String>>()
                            .join("\n"),
                        if meta.inner.is_empty() { "" } else { "\n" }
                    )
                })
                .unwrap_or_default(),
            self.elements
                .iter()
                .map(|el| format!("{el};"))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.elements.is_empty() { "" } else { "\n" }
        )
    }
}

impl term::Display for Block {
    fn display(&self, term: &mut Term) {
        if let Some(meta) = self.meta.as_ref() {
            meta.display(term);
        }
    }
}

impl Operator for Block {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let mut output: Option<AnyValue> = None;
            for element in self.elements.iter() {
                output = element.execute(owner, components, args, cx).await?;
                if self.by_first && output.is_some() {
                    return Ok(output);
                }
            }
            // Block always should return some value.
            Ok(if output.is_none() {
                Some(AnyValue::new(()))
            } else {
                output
            })
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::Block,
        inf::tests,
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::unbound(format!(
            "[{}]\n[{}]\n[{}]\n[{}]\n[{}]\n[{}]",
            include_str!("../tests/reading/if.sibs"),
            include_str!("../tests/reading/variable_assignation.sibs"),
            include_str!("../tests/reading/function.sibs"),
            include_str!("../tests/reading/optional.sibs"),
            include_str!("../tests/reading/each.sibs"),
            include_str!("../tests/reading/refs.sibs")
        ));
        while let Some(entity) = Block::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&entity.to_string())
            );
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), E> {
        let mut reader = Reader::unbound(format!(
            "[{}]\n[{}]\n[{}]\n[{}]\n[{}]\n[{}]",
            include_str!("../tests/reading/if.sibs"),
            include_str!("../tests/reading/variable_assignation.sibs"),
            include_str!("../tests/reading/function.sibs"),
            include_str!("../tests/reading/optional.sibs"),
            include_str!("../tests/reading/each.sibs"),
            include_str!("../tests/reading/refs.sibs")
        ));
        while let Some(entity) = Block::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&entity.to_string()),
                reader.get_fragment(&entity.token)?.lined
            );
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        entry::{
            block::{Block, Element},
            command::Command,
            embedded::{each::Each, If::If},
            function::Function,
            meta::Meta,
            optional::Optional,
            reference::Reference,
            task::Task,
            variable_assignation::VariableAssignation,
        },
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Element {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            let permissions = scope.read().unwrap().permissions();
            let mut allowed = vec![Command::arbitrary_with(scope.clone())
                .prop_map(Element::Command)
                .boxed()];
            if permissions.func {
                allowed.push(
                    Function::arbitrary_with(scope.clone())
                        .prop_map(Element::Function)
                        .boxed(),
                );
            }
            if permissions.each {
                allowed.push(
                    Each::arbitrary_with(scope.clone())
                        .prop_map(Element::Each)
                        .boxed(),
                );
            }
            if permissions.r#if {
                allowed.push(
                    If::arbitrary_with(scope.clone())
                        .prop_map(Element::If)
                        .boxed(),
                );
            }
            if permissions.optional {
                allowed.push(
                    Optional::arbitrary_with(scope.clone())
                        .prop_map(Element::Optional)
                        .boxed(),
                );
            }
            if permissions.variable_assignation {
                allowed.push(
                    VariableAssignation::arbitrary_with(scope.clone())
                        .prop_map(Element::VariableAssignation)
                        .boxed(),
                );
            }
            if permissions.reference {
                allowed.push(
                    Reference::arbitrary_with(scope.clone())
                        .prop_map(Element::Reference)
                        .boxed(),
                );
            }
            prop::strategy::Union::new(allowed).boxed()
        }
    }

    impl Arbitrary for Block {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Block);
            let boxed = (
                prop::collection::vec(Element::arbitrary_with(scope.clone()), 1..=10),
                Meta::arbitrary_with(scope.clone()),
            )
                .prop_map(|(elements, meta)| Block {
                    elements,
                    meta: Some(meta),
                    token: 0,
                    by_first: false,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Block);
            boxed
        }
    }

    fn reading(block: Block) -> Result<(), E> {
        async_io::block_on(async {
            let origin = format!("test {block};");
            let mut reader = Reader::unbound(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
            }
            Ok(())
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn test_run_task(
            args in any_with::<Block>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
