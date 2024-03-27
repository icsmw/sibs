mod block;
mod comment;
mod component;
mod conditions;
mod function;
mod meta;
mod optional;
mod primitives;
mod reference;
mod statements;
mod string;
mod task;
mod values;
mod variable;

pub use block::*;
pub use comment::*;
pub use component::*;
pub use conditions::*;
pub use function::*;
pub use meta::*;
pub use optional::*;
pub use primitives::*;
pub use reference::*;
pub use statements::{each::*, first::*, If::*};
pub use string::{command::*, pattern::*, simple::*};
pub use task::*;
pub use values::*;
pub use variable::*;

use crate::{
    error::LinkedErr,
    inf::{term, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Term},
    reader::{Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElTarget {
    Function,
    If,
    Each,
    First,
    VariableAssignation,
    Optional,
    Reference,
    PatternString,
    VariableName,
    Comparing,
    Combination,
    Subsequence,
    Condition,
    Values,
    Block,
    Meta,
    Command,
    Task,
    Component,
    Integer,
    Boolean,
    VariableDeclaration,
    VariableVariants,
    VariableType,
    SimpleString,
    Comment,
}

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub comments: Vec<Comment>,
}

impl Metadata {
    pub fn empty() -> Self {
        Metadata {
            comments: Vec::new(),
        }
    }
}

impl Formation for Metadata {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            self.comments
                .iter()
                .map(|c| format!("{}{c}", cursor.offset_as_string()))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.comments.is_empty() { "" } else { "\n" },
        )
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    Function(Function, Metadata),
    If(If, Metadata),
    Each(Each, Metadata),
    First(First, Metadata),
    VariableAssignation(VariableAssignation, Metadata),
    Optional(Optional, Metadata),
    Reference(Reference, Metadata),
    PatternString(PatternString, Metadata),
    VariableName(VariableName, Metadata),
    Comparing(Comparing, Metadata),
    Combination(Combination, Metadata),
    Subsequence(Subsequence, Metadata),
    Condition(Condition, Metadata),
    Values(Values, Metadata),
    Block(Block, Metadata),
    Meta(Meta, Metadata),
    Command(Command, Metadata),
    Task(Task, Metadata),
    Component(Component, Metadata),
    Boolean(Boolean, Metadata),
    Integer(Integer, Metadata),
    VariableDeclaration(VariableDeclaration, Metadata),
    VariableVariants(VariableVariants, Metadata),
    VariableType(VariableType, Metadata),
    SimpleString(SimpleString, Metadata),
    Comment(Comment),
}

impl Element {
    fn parse(
        reader: &mut Reader,
        targets: &[ElTarget],
        includes: bool,
    ) -> Result<Option<Element>, LinkedErr<E>> {
        let mut comments: Vec<Comment> = vec![];
        while let Some(comment) = Comment::read(reader)? {
            comments.push(comment);
        }
        let md = Metadata { comments };
        if includes == targets.contains(&ElTarget::Combination) {
            if let Some(el) = Combination::read(reader)? {
                return Ok(Some(Element::Combination(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Subsequence) {
            if let Some(el) = Subsequence::read(reader)? {
                return Ok(Some(Element::Subsequence(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Condition) {
            if let Some(el) = Condition::read(reader)? {
                return Ok(Some(Element::Condition(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Meta) {
            if let Some(el) = Meta::read(reader)? {
                return Ok(Some(Element::Meta(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Command) {
            if let Some(el) = Command::read(reader)? {
                return Ok(Some(Element::Command(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::If) {
            if let Some(el) = If::read(reader)? {
                return Ok(Some(Element::If(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Optional) {
            if let Some(el) = Optional::read(reader)? {
                return Ok(Some(Element::Optional(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Comparing) {
            if let Some(el) = Comparing::read(reader)? {
                return Ok(Some(Element::Comparing(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Integer) {
            if let Some(el) = Integer::read(reader)? {
                return Ok(Some(Element::Integer(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Boolean) {
            if let Some(el) = Boolean::read(reader)? {
                return Ok(Some(Element::Boolean(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Function) {
            if let Some(el) = Function::read(reader)? {
                return Ok(Some(Element::Function(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::VariableAssignation) {
            if let Some(el) = VariableAssignation::read(reader)? {
                return Ok(Some(Element::VariableAssignation(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::VariableName) {
            if let Some(el) = VariableName::read(reader)? {
                return Ok(Some(Element::VariableName(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Each) {
            if let Some(el) = Each::read(reader)? {
                return Ok(Some(Element::Each(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::First) {
            if let Some(el) = First::read(reader)? {
                return Ok(Some(Element::First(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Reference) {
            if let Some(el) = Reference::read(reader)? {
                return Ok(Some(Element::Reference(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::PatternString) {
            if let Some(el) = PatternString::read(reader)? {
                return Ok(Some(Element::PatternString(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Block) {
            if let Some(el) = Block::read(reader)? {
                return Ok(Some(Element::Block(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Values) {
            if let Some(el) = Values::read(reader)? {
                return Ok(Some(Element::Values(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Component) {
            if let Some(el) = Component::read(reader)? {
                return Ok(Some(Element::Component(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::Task) {
            if let Some(el) = Task::read(reader)? {
                return Ok(Some(Element::Task(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::VariableDeclaration) {
            if let Some(el) = VariableDeclaration::read(reader)? {
                return Ok(Some(Element::VariableDeclaration(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::VariableType) {
            if let Some(el) = VariableType::read(reader)? {
                return Ok(Some(Element::VariableType(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::VariableVariants) {
            if let Some(el) = VariableVariants::read(reader)? {
                return Ok(Some(Element::VariableVariants(el, md)));
            }
        }
        if includes == targets.contains(&ElTarget::SimpleString) {
            if let Some(el) = SimpleString::read(reader)? {
                return Ok(Some(Element::SimpleString(el, md)));
            }
        }
        Ok(None)
    }

    pub fn exclude(
        reader: &mut Reader,
        targets: &[ElTarget],
    ) -> Result<Option<Element>, LinkedErr<E>> {
        Self::parse(reader, targets, false)
    }

    pub fn include(
        reader: &mut Reader,
        targets: &[ElTarget],
    ) -> Result<Option<Element>, LinkedErr<E>> {
        Self::parse(reader, targets, true)
    }

    pub fn get_metadata(&self) -> &Metadata {
        match self {
            Self::Function(_, md) => md,
            Self::If(_, md) => md,
            Self::Each(_, md) => md,
            Self::First(_, md) => md,
            Self::VariableAssignation(_, md) => md,
            Self::Comparing(_, md) => md,
            Self::Combination(_, md) => md,
            Self::Condition(_, md) => md,
            Self::Subsequence(_, md) => md,
            Self::Optional(_, md) => md,
            Self::Reference(_, md) => md,
            Self::PatternString(_, md) => md,
            Self::VariableName(_, md) => md,
            Self::Values(_, md) => md,
            Self::Meta(_, md) => md,
            Self::Block(_, md) => md,
            Self::Command(_, md) => md,
            Self::Task(_, md) => md,
            Self::Component(_, md) => md,
            Self::Boolean(_, md) => md,
            Self::Integer(_, md) => md,
            Self::VariableDeclaration(_, md) => md,
            Self::VariableVariants(_, md) => md,
            Self::VariableType(_, md) => md,
            Self::SimpleString(_, md) => md,
            Self::Comment(_) => {
                panic!("Comment doesn't have metadata");
            }
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function(v, _) => v.to_string(),
                Self::If(v, _) => v.to_string(),
                Self::Each(v, _) => v.to_string(),
                Self::First(v, _) => v.to_string(),
                Self::VariableAssignation(v, _) => v.to_string(),
                Self::Comparing(v, _) => v.to_string(),
                Self::Combination(v, _) => v.to_string(),
                Self::Condition(v, _) => v.to_string(),
                Self::Subsequence(v, _) => v.to_string(),
                Self::Optional(v, _) => v.to_string(),
                Self::Reference(v, _) => v.to_string(),
                Self::PatternString(v, _) => v.to_string(),
                Self::VariableName(v, _) => v.to_string(),
                Self::Values(v, _) => v.to_string(),
                Self::Meta(v, _) => v.to_string(),
                Self::Block(v, _) => v.to_string(),
                Self::Command(v, _) => v.to_string(),
                Self::Task(v, _) => v.to_string(),
                Self::Component(v, _) => v.to_string(),
                Self::Boolean(v, _) => v.to_string(),
                Self::Integer(v, _) => v.to_string(),
                Self::VariableDeclaration(v, _) => v.to_string(),
                Self::VariableVariants(v, _) => v.to_string(),
                Self::VariableType(v, _) => v.to_string(),
                Self::SimpleString(v, _) => v.to_string(),
                Self::Comment(v) => v.to_string(),
            }
        )
    }
}

impl Formation for Element {
    fn elements_count(&self) -> usize {
        match self {
            Self::Function(v, _) => v.elements_count(),
            Self::If(v, _) => v.elements_count(),
            Self::Each(v, _) => v.elements_count(),
            Self::First(v, _) => v.elements_count(),
            Self::VariableAssignation(v, _) => v.elements_count(),
            Self::Comparing(v, _) => v.elements_count(),
            Self::Combination(v, _) => v.elements_count(),
            Self::Condition(v, _) => v.elements_count(),
            Self::Subsequence(v, _) => v.elements_count(),
            Self::Optional(v, _) => v.elements_count(),
            Self::Reference(v, _) => v.elements_count(),
            Self::PatternString(v, _) => v.elements_count(),
            Self::VariableName(v, _) => v.elements_count(),
            Self::Values(v, _) => v.elements_count(),
            Self::Meta(v, _) => v.elements_count(),
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
            Self::Comment(v) => v.elements_count(),
        }
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        fn format_el<A, B>(el: &A, md: &B, cursor: &mut FormationCursor) -> String
        where
            A: Formation,
            B: Formation,
        {
            format!("{}{}", md.format(cursor), el.format(cursor))
        }
        match self {
            Self::Function(v, m) => format_el(v, m, cursor),
            Self::If(v, m) => format_el(v, m, cursor),
            Self::Each(v, m) => format_el(v, m, cursor),
            Self::First(v, m) => format_el(v, m, cursor),
            Self::VariableAssignation(v, m) => format_el(v, m, cursor),
            Self::Comparing(v, m) => format_el(v, m, cursor),
            Self::Combination(v, m) => format_el(v, m, cursor),
            Self::Condition(v, m) => format_el(v, m, cursor),
            Self::Subsequence(v, m) => format_el(v, m, cursor),
            Self::Optional(v, m) => format_el(v, m, cursor),
            Self::Reference(v, m) => format_el(v, m, cursor),
            Self::PatternString(v, m) => format_el(v, m, cursor),
            Self::VariableName(v, m) => format_el(v, m, cursor),
            Self::Values(v, m) => format_el(v, m, cursor),
            Self::Meta(v, m) => format_el(v, m, cursor),
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
            Self::Comment(v) => v.format(cursor),
        }
    }
}
impl term::Display for Element {
    fn display(&self, _term: &mut Term) {
        // term.print_fmt(&self.as_lines());
    }
}

impl Operator for Element {
    fn token(&self) -> usize {
        match self {
            Self::Function(v, _) => v.token(),
            Self::If(v, _) => v.token(),
            Self::Each(v, _) => v.token(),
            Self::First(v, _) => v.token(),
            Self::VariableAssignation(v, _) => v.token(),
            Self::Comparing(v, _) => v.token(),
            Self::Combination(v, _) => v.token(),
            Self::Condition(v, _) => v.token(),
            Self::Subsequence(v, _) => v.token(),
            Self::Optional(v, _) => v.token(),
            Self::Reference(v, _) => v.token(),
            Self::PatternString(v, _) => v.token(),
            Self::VariableName(v, _) => v.token(),
            Self::Values(v, _) => v.token(),
            Self::Block(v, _) => v.token(),
            Self::Command(v, _) => v.token(),
            Self::Task(v, _) => v.token(),
            Self::Component(v, _) => v.token(),
            Self::Integer(v, _) => v.token(),
            Self::Boolean(v, _) => v.token(),
            Self::VariableDeclaration(v, _) => v.token,
            Self::VariableVariants(v, _) => v.token,
            Self::Meta(v, _) => v.token,
            Self::VariableType(v, _) => v.token,
            Self::SimpleString(v, _) => v.token(),
            Self::Comment(v) => v.token,
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
                Self::Function(v, _) => v.execute(owner, components, args, cx).await,
                Self::If(v, _) => v.execute(owner, components, args, cx).await,
                Self::Each(v, _) => v.execute(owner, components, args, cx).await,
                Self::First(v, _) => v.execute(owner, components, args, cx).await,
                Self::VariableAssignation(v, _) => v.execute(owner, components, args, cx).await,
                Self::Comparing(v, _) => v.execute(owner, components, args, cx).await,
                Self::Combination(v, _) => v.execute(owner, components, args, cx).await,
                Self::Condition(v, _) => v.execute(owner, components, args, cx).await,
                Self::Subsequence(v, _) => v.execute(owner, components, args, cx).await,
                Self::Optional(v, _) => v.execute(owner, components, args, cx).await,
                Self::Reference(v, _) => v.execute(owner, components, args, cx).await,
                Self::PatternString(v, _) => v.execute(owner, components, args, cx).await,
                Self::VariableName(v, _) => v.execute(owner, components, args, cx).await,
                Self::Values(v, _) => v.execute(owner, components, args, cx).await,
                Self::Block(v, _) => v.execute(owner, components, args, cx).await,
                Self::Command(v, _) => v.execute(owner, components, args, cx).await,
                Self::Task(v, _) => v.execute(owner, components, args, cx).await,
                Self::Component(v, _) => v.execute(owner, components, args, cx).await,
                Self::Integer(v, _) => v.execute(owner, components, args, cx).await,
                Self::Boolean(v, _) => v.execute(owner, components, args, cx).await,
                Self::Meta(..) => Ok(None),
                Self::VariableDeclaration(..) => Ok(None),
                Self::VariableVariants(..) => Ok(None),
                Self::VariableType(..) => Ok(None),
                Self::SimpleString(v, _) => v.execute(owner, components, args, cx).await,
                Self::Comment(_) => Ok(None),
            }
        })
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{
            Block, Boolean, Combination, Command, Comment, Comparing, Component, Condition, Each,
            ElTarget, Element, First, Function, If, Integer, Meta, Metadata, Optional,
            PatternString, Reference, SimpleString, Subsequence, Task, Values, VariableAssignation,
            VariableDeclaration, VariableName, VariableType, VariableVariants,
        },
        inf::{operator::E, tests::*, Context},
        reader::Reading,
    };
    use proptest::prelude::*;

    fn generate(targets: &[ElTarget], deep: usize) -> Vec<BoxedStrategy<Element>> {
        let mut collected = vec![];
        if targets.contains(&ElTarget::Combination) {
            collected.push(
                Combination::arbitrary()
                    .prop_map(|el| Element::Combination(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Subsequence) {
            collected.push(
                Subsequence::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Subsequence(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Condition) {
            collected.push(
                Condition::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Condition(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Integer) {
            collected.push(
                Integer::arbitrary()
                    .prop_map(|el| Element::Integer(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Boolean) {
            collected.push(
                Boolean::arbitrary()
                    .prop_map(|el| Element::Boolean(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Block) {
            collected.push(
                Block::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Block(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Command) {
            collected.push(
                Command::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Command(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Comparing) {
            collected.push(
                Comparing::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Comparing(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Component) {
            collected.push(
                Component::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Component(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Each) {
            collected.push(
                Each::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Each(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::First) {
            collected.push(
                First::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::First(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Function) {
            collected.push(
                Function::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Function(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::If) {
            collected.push(
                If::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::If(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Meta) {
            collected.push(
                Meta::arbitrary()
                    .prop_map(|el| Element::Meta(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Optional) {
            collected.push(
                Optional::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Optional(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::PatternString) {
            collected.push(
                PatternString::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::PatternString(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Reference) {
            collected.push(
                Reference::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Reference(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Task) {
            collected.push(
                Task::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Task(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Values) {
            collected.push(
                Values::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Values(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableAssignation) {
            collected.push(
                VariableAssignation::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::VariableAssignation(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableName) {
            collected.push(
                VariableName::arbitrary()
                    .prop_map(|el| Element::VariableName(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableType) {
            collected.push(
                VariableType::arbitrary()
                    .prop_map(|el| Element::VariableType(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableDeclaration) {
            collected.push(
                VariableDeclaration::arbitrary_with(deep)
                    .prop_map(|el| Element::VariableDeclaration(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableVariants) {
            collected.push(
                VariableVariants::arbitrary()
                    .prop_map(|el| Element::VariableVariants(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::SimpleString) {
            collected.push(
                SimpleString::arbitrary()
                    .prop_map(|el| Element::SimpleString(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Comment) {
            collected.push(Comment::arbitrary().prop_map(Element::Comment).boxed());
        }
        collected
    }

    impl Arbitrary for Element {
        type Parameters = (Vec<ElTarget>, usize);
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with((targets, deep): Self::Parameters) -> Self::Strategy {
            prop::strategy::Union::new(generate(&targets, deep)).boxed()
        }
    }

    fn reading(el: Element) -> Result<(), E> {
        get_rt().block_on(async {
            let mut cx: Context = Context::create().unbound()?;
            let origin = format!("{el};");
            let mut reader = cx.reader().from_str(&origin);
            while let Some(block) = Block::read(&mut reader)? {
                assert_eq!(format!("{block};"), origin);
            }
            Ok(())
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            max_shrink_iters: 5000,
            ..ProptestConfig::with_cases(10)
        })]
        #[test]
        fn test_run_task(
            args in any_with::<Element>((vec![ElTarget::Function], 0))
        ) {
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
