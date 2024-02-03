use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
        term::{self, Term},
    },
    reader::{
        chars,
        entry::{
            Command, Component, Each, Function, If, Meta, Optional, Reading, Reference,
            VariableAssignation,
        },
        Reader, E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Element {
    Function(Function),
    If(If),
    Each(Each),
    VariableAssignation(VariableAssignation),
    Optional(Optional),
    Reference(Reference),
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
                Self::VariableAssignation(v) => v.to_string(),
                Self::Optional(v) => v.to_string(),
                Self::Reference(v) => v.to_string(),
            }
        )
    }
}

impl Operator for Element {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::Command(v) => v.process(owner, components, args, cx).await,
                Self::Function(v) => v.process(owner, components, args, cx).await,
                Self::If(v) => v.process(owner, components, args, cx).await,
                Self::Each(v) => v.process(owner, components, args, cx).await,
                Self::VariableAssignation(v) => v.process(owner, components, args, cx).await,
                Self::Optional(v) => v.process(owner, components, args, cx).await,
                Self::Reference(v) => v.process(owner, components, args, cx).await,
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
        let mut elements: Vec<Element> = vec![];
        let mut meta: Option<Meta> = None;
        while !reader.rest().trim().is_empty() {
            if let Some(md) = Meta::read(reader)? {
                meta = Some(md);
                continue;
            }
            if let Some(el) = If::read(reader)? {
                elements.push(Element::If(el));
                continue;
            }
            if let Some(el) = Optional::read(reader)? {
                elements.push(Element::Optional(el));
                continue;
            }
            if let Some(el) = VariableAssignation::read(reader)? {
                elements.push(Element::VariableAssignation(el));
                continue;
            }
            if let Some(el) = Each::read(reader)? {
                elements.push(Element::Each(el));
                continue;
            }
            if let Some(el) = Reference::read(reader)? {
                elements.push(Element::Reference(el));
                continue;
            }
            if let Some(el) = Function::read(reader)? {
                elements.push(Element::Function(el));
                continue;
            }
            if let Some((cmd, _)) = reader.until().char(&[&chars::SEMICOLON]) {
                reader.move_to().next();
                elements.push(Element::Command(Command::new(cmd, reader.token()?.id)));
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
                token: reader.token()?.id,
                by_first: false,
            })
        })
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
                .map(|el| format!(
                    "{el}{}",
                    match el {
                        Element::Function(_) | Element::Command(_) => ";",
                        _ => "",
                    }
                ))
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
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let mut output: Option<AnyValue> = None;
            for element in self.elements.iter() {
                output = element.process(owner, components, args, cx).await?;
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
mod test_blocks {
    use crate::reader::{
        entry::{Block, Reading},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            include_str!("../../tests/normal/if.sibs"),
            include_str!("../../tests/normal/variable_assignation.sibs"),
            include_str!("../../tests/normal/function.sibs"),
            include_str!("../../tests/normal/optional.sibs"),
            include_str!("../../tests/normal/each.sibs"),
            include_str!("../../tests/normal/refs.sibs")
        ));
        while let Some(entity) = Block::read(&mut reader)? {
            assert_eq!(
                format!("[{}]", tests::trim(reader.recent())),
                tests::trim(&entity.to_string())
            );
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        inf::tests::*,
        reader::entry::{
            block::{Block, Element},
            command::Command,
            embedded::{each::Each, If::If},
            function::Function,
            meta::Meta,
            optional::Optional,
            reference::Reference,
            variable_assignation::VariableAssignation,
        },
    };
    use proptest::prelude::*;

    impl Arbitrary for Element {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Each::arbitrary_with(scope.clone()).prop_map(Element::Each),
                If::arbitrary_with(scope.clone()).prop_map(Element::If),
                Command::arbitrary_with(scope.clone()).prop_map(Element::Command),
                Optional::arbitrary_with(scope.clone()).prop_map(Element::Optional),
                VariableAssignation::arbitrary_with(scope.clone())
                    .prop_map(Element::VariableAssignation),
                Reference::arbitrary_with(scope.clone()).prop_map(Element::Reference),
                Function::arbitrary_with(scope.clone()).prop_map(Element::Function),
            ]
            .boxed()
        }
    }

    impl Arbitrary for Block {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            (
                prop::collection::vec(Element::arbitrary_with(scope.clone()), 1..=10),
                Meta::arbitrary_with(scope.clone()),
            )
                .prop_map(|(elements, meta)| Block {
                    elements,
                    meta: Some(meta),
                    token: 0,
                    by_first: false,
                })
                .boxed()
        }
    }
}
