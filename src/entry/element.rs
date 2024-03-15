use crate::{
    entry::{
        Block, Boolean, Command, Comparing, Component, Each, First, Function, If, Integer, Meta,
        Optional, PatternString, Reference, SimpleString, Task, Values, VariableAssignation,
        VariableName,
    },
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
        term::{self, Term},
    },
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
    Values,
    Block,
    Meta,
    Command,
    Task,
    Component,
    Integer,
    Boolean,
}

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
    Comparing(Comparing),
    Values(Values),
    Block(Block),
    Meta(Meta),
    Command(Command),
    Task(Task),
    Component(Component),
    Boolean(Boolean),
    Integer(Integer),
}

impl Element {
    fn parse(
        reader: &mut Reader,
        targets: &[ElTarget],
        includes: bool,
    ) -> Result<Option<Element>, LinkedErr<E>> {
        if includes == targets.contains(&ElTarget::Integer) {
            if let Some(el) = Integer::read(reader)? {
                return Ok(Some(Element::Integer(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Boolean) {
            if let Some(el) = Boolean::read(reader)? {
                return Ok(Some(Element::Boolean(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Meta) {
            if let Some(el) = Meta::read(reader)? {
                return Ok(Some(Element::Meta(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Command) {
            if let Some(el) = Command::read(reader)? {
                return Ok(Some(Element::Command(el)));
            }
        }
        if includes == targets.contains(&ElTarget::If) {
            if let Some(el) = If::read(reader)? {
                return Ok(Some(Element::If(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Optional) {
            if let Some(el) = Optional::read(reader)? {
                return Ok(Some(Element::Optional(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Comparing) {
            if let Some(el) = Comparing::read(reader)? {
                return Ok(Some(Element::Comparing(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Function) {
            if let Some(el) = Function::read(reader)? {
                return Ok(Some(Element::Function(el)));
            }
        }
        if includes == targets.contains(&ElTarget::VariableAssignation) {
            if let Some(el) = VariableAssignation::read(reader)? {
                return Ok(Some(Element::VariableAssignation(el)));
            }
        }
        if includes == targets.contains(&ElTarget::VariableName) {
            if let Some(el) = VariableName::read(reader)? {
                return Ok(Some(Element::VariableName(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Each) {
            if let Some(el) = Each::read(reader)? {
                return Ok(Some(Element::Each(el)));
            }
        }
        if includes == targets.contains(&ElTarget::First) {
            if let Some(el) = First::read(reader)? {
                return Ok(Some(Element::First(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Reference) {
            if let Some(el) = Reference::read(reader)? {
                return Ok(Some(Element::Reference(el)));
            }
        }
        if includes == targets.contains(&ElTarget::PatternString) {
            if let Some(el) = PatternString::read(reader)? {
                return Ok(Some(Element::PatternString(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Block) {
            if let Some(el) = Block::read(reader)? {
                return Ok(Some(Element::Block(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Values) {
            if let Some(el) = Values::read(reader)? {
                return Ok(Some(Element::Values(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Component) {
            if let Some(el) = Component::read(reader)? {
                return Ok(Some(Element::Component(el)));
            }
        }
        if includes == targets.contains(&ElTarget::Task) {
            if let Some(el) = Task::read(reader)? {
                return Ok(Some(Element::Task(el)));
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
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function(v) => v.to_string(),
                Self::If(v) => v.to_string(),
                Self::Each(v) => v.to_string(),
                Self::First(v) => v.to_string(),
                Self::VariableAssignation(v) => v.to_string(),
                Self::Comparing(v) => v.to_string(),
                Self::Optional(v) => v.to_string(),
                Self::Reference(v) => v.to_string(),
                Self::PatternString(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
                Self::Values(v) => v.to_string(),
                Self::Meta(v) => v.to_string(),
                Self::Block(v) => v.to_string(),
                Self::Command(v) => v.to_string(),
                Self::Task(v) => v.to_string(),
                Self::Component(v) => v.to_string(),
                Self::Boolean(v) => v.to_string(),
                Self::Integer(v) => v.to_string(),
            }
        )
    }
}

impl term::Display for Element {
    fn display(&self, term: &mut Term) {
        // term.print_fmt(&self.as_lines());
    }
}

impl Operator for Element {
    fn token(&self) -> usize {
        match self {
            Self::Function(v) => v.token(),
            Self::If(v) => v.token(),
            Self::Each(v) => v.token(),
            Self::First(v) => v.token(),
            Self::VariableAssignation(v) => v.token(),
            Self::Comparing(v) => v.token(),
            Self::Optional(v) => v.token(),
            Self::Reference(v) => v.token(),
            Self::PatternString(v) => v.token(),
            Self::VariableName(v) => v.token(),
            Self::Values(v) => v.token(),
            Self::Block(v) => v.token(),
            Self::Meta(v) => v.token,
            Self::Command(v) => v.token(),
            Self::Task(v) => v.token(),
            Self::Component(v) => v.token(),
            Self::Integer(v) => v.token(),
            Self::Boolean(v) => v.token(),
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
                Self::Function(v) => v.execute(owner, components, args, cx).await,
                Self::If(v) => v.execute(owner, components, args, cx).await,
                Self::Each(v) => v.execute(owner, components, args, cx).await,
                Self::First(v) => v.execute(owner, components, args, cx).await,
                Self::VariableAssignation(v) => v.execute(owner, components, args, cx).await,
                Self::Comparing(v) => v.execute(owner, components, args, cx).await,
                Self::Optional(v) => v.execute(owner, components, args, cx).await,
                Self::Reference(v) => v.execute(owner, components, args, cx).await,
                Self::PatternString(v) => v.execute(owner, components, args, cx).await,
                Self::VariableName(v) => v.execute(owner, components, args, cx).await,
                Self::Values(v) => v.execute(owner, components, args, cx).await,
                Self::Block(v) => v.execute(owner, components, args, cx).await,
                Self::Command(v) => v.execute(owner, components, args, cx).await,
                Self::Task(v) => v.execute(owner, components, args, cx).await,
                Self::Component(v) => v.execute(owner, components, args, cx).await,
                Self::Integer(v) => v.execute(owner, components, args, cx).await,
                Self::Boolean(v) => v.execute(owner, components, args, cx).await,
                Self::Meta(_) => Ok(None),
            }
        })
    }
}

impl Reading<Element> for Element {
    fn read(reader: &mut Reader) -> Result<Option<Element>, LinkedErr<E>> {
        Ok(if let Some(el) = Integer::read(reader)? {
            Some(Element::Integer(el))
        } else if let Some(el) = Boolean::read(reader)? {
            Some(Element::Boolean(el))
        } else if let Some(el) = Meta::read(reader)? {
            Some(Element::Meta(el))
        } else if let Some(el) = Command::read(reader)? {
            Some(Element::Command(el))
        } else if let Some(el) = If::read(reader)? {
            Some(Element::If(el))
        } else if let Some(el) = Optional::read(reader)? {
            Some(Element::Optional(el))
        } else if let Some(el) = Comparing::read(reader)? {
            Some(Element::Comparing(el))
        } else if let Some(el) = Function::read(reader)? {
            Some(Element::Function(el))
        } else if let Some(el) = VariableAssignation::read(reader)? {
            Some(Element::VariableAssignation(el))
        } else if let Some(el) = VariableName::read(reader)? {
            Some(Element::VariableName(el))
        } else if let Some(el) = Each::read(reader)? {
            Some(Element::Each(el))
        } else if let Some(el) = First::read(reader)? {
            Some(Element::First(el))
        } else if let Some(el) = Reference::read(reader)? {
            Some(Element::Reference(el))
        } else if let Some(el) = PatternString::read(reader)? {
            Some(Element::PatternString(el))
        } else if let Some(el) = Block::read(reader)? {
            Some(Element::Block(el))
        } else if let Some(el) = Component::read(reader)? {
            Some(Element::Component(el))
        } else if let Some(el) = Task::read(reader)? {
            Some(Element::Task(el))
        } else {
            Values::read(reader)?.map(Element::Values)
        })
    }
}

#[derive(Debug, Clone)]
pub enum ElementExd {
    Element(Element),
    SimpleString(SimpleString),
}

impl fmt::Display for ElementExd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SimpleString(s) => s.to_string(),
                Self::Element(v) => v.to_string(),
            }
        )
    }
}

impl Operator for ElementExd {
    fn token(&self) -> usize {
        match self {
            Self::SimpleString(v) => v.token,
            Self::Element(v) => v.token(),
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
                Self::Element(v) => v.execute(owner, components, args, cx).await,
                Self::SimpleString(v) => Ok(Some(AnyValue::new(v.value.to_owned()))),
            }
        })
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        entry::{
            Block, Boolean, Command, Comparing, Component, Each, ElTarget, Element, ElementExd,
            First, Function, If, Integer, Meta, Optional, PatternString, Reference, SimpleString,
            Task, Values, VariableAssignation, VariableName,
        },
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use lazy_static::lazy_static;
    use proptest::prelude::*;
    use std::sync::RwLock;

    lazy_static! {
        static ref FUNCTION: RwLock<i16> = RwLock::new(100);
        static ref IF: RwLock<i16> = RwLock::new(50);
        static ref EACH: RwLock<i16> = RwLock::new(10);
        static ref FIRST: RwLock<i16> = RwLock::new(10);
        static ref VARIABLE_ASSIGNATION: RwLock<i16> = RwLock::new(200);
        static ref OPTIONAL: RwLock<i16> = RwLock::new(200);
        static ref REFERENCE: RwLock<i16> = RwLock::new(500);
        static ref PATTERN_STRING: RwLock<i16> = RwLock::new(5000);
        static ref VARIABLE_NAME: RwLock<i16> = RwLock::new(5000);
        static ref COMPARING: RwLock<i16> = RwLock::new(200);
        static ref VALUES: RwLock<i16> = RwLock::new(200);
        static ref BLOCK: RwLock<i16> = RwLock::new(100);
        static ref META: RwLock<i16> = RwLock::new(1000);
        static ref COMMAND: RwLock<i16> = RwLock::new(200);
        static ref TASK: RwLock<i16> = RwLock::new(50);
        static ref COMPONENT: RwLock<i16> = RwLock::new(50);
        static ref INTEGER: RwLock<i16> = RwLock::new(10000);
        static ref BOOLEAN: RwLock<i16> = RwLock::new(10000);
    }

    impl Arbitrary for ElementExd {
        type Parameters = Vec<ElTarget>;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(targets: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                SimpleString::arbitrary().prop_map(ElementExd::SimpleString),
                Element::arbitrary_with(targets).prop_map(ElementExd::Element),
            ]
            .boxed()
        }
    }
    fn generate(targets: &[ElTarget], no_limits: bool) -> Vec<BoxedStrategy<Element>> {
        let mut collected = vec![];
        if targets.contains(&ElTarget::Integer) && (*BLOCK.read().unwrap() > 0 || no_limits) {
            *INTEGER.write().unwrap() -= 1;
            collected.push(Integer::arbitrary().prop_map(Element::Integer).boxed());
        }
        if targets.contains(&ElTarget::Boolean) && (*BLOCK.read().unwrap() > 0 || no_limits) {
            *BOOLEAN.write().unwrap() -= 1;
            collected.push(Boolean::arbitrary().prop_map(Element::Boolean).boxed());
        }
        if targets.contains(&ElTarget::Block) && (*BLOCK.read().unwrap() > 0 || no_limits) {
            *BLOCK.write().unwrap() -= 1;
            collected.push(Block::arbitrary().prop_map(Element::Block).boxed());
        }
        if targets.contains(&ElTarget::Command) && (*COMMAND.read().unwrap() > 0 || no_limits) {
            *COMMAND.write().unwrap() -= 1;
            collected.push(Command::arbitrary().prop_map(Element::Command).boxed());
        }
        if targets.contains(&ElTarget::Comparing) && (*COMPARING.read().unwrap() > 0 || no_limits) {
            *COMPARING.write().unwrap() -= 1;
            collected.push(Comparing::arbitrary().prop_map(Element::Comparing).boxed());
        }
        if targets.contains(&ElTarget::Component) && (*COMPONENT.read().unwrap() > 0 || no_limits) {
            *COMPONENT.write().unwrap() -= 1;
            collected.push(Component::arbitrary().prop_map(Element::Component).boxed());
        }
        if targets.contains(&ElTarget::Each) && (*EACH.read().unwrap() > 0 || no_limits) {
            *EACH.write().unwrap() -= 1;
            collected.push(Each::arbitrary().prop_map(Element::Each).boxed());
        }
        if targets.contains(&ElTarget::First) && (*FIRST.read().unwrap() > 0 || no_limits) {
            *FIRST.write().unwrap() -= 1;
            collected.push(First::arbitrary().prop_map(Element::First).boxed());
        }
        if targets.contains(&ElTarget::Function) && (*FUNCTION.read().unwrap() > 0 || no_limits) {
            *FUNCTION.write().unwrap() -= 1;
            collected.push(Function::arbitrary().prop_map(Element::Function).boxed());
        }
        if targets.contains(&ElTarget::If) && (*IF.read().unwrap() > 0 || no_limits) {
            *IF.write().unwrap() -= 1;
            collected.push(If::arbitrary().prop_map(Element::If).boxed());
        }
        if targets.contains(&ElTarget::Meta) && (*META.read().unwrap() > 0 || no_limits) {
            *META.write().unwrap() -= 1;
            collected.push(Meta::arbitrary().prop_map(Element::Meta).boxed());
        }
        if targets.contains(&ElTarget::Optional) && (*OPTIONAL.read().unwrap() > 0 || no_limits) {
            *OPTIONAL.write().unwrap() -= 1;
            collected.push(Optional::arbitrary().prop_map(Element::Optional).boxed());
        }
        if targets.contains(&ElTarget::PatternString)
            && (*PATTERN_STRING.read().unwrap() > 0 || no_limits)
        {
            *PATTERN_STRING.write().unwrap() -= 1;
            collected.push(
                PatternString::arbitrary()
                    .prop_map(Element::PatternString)
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Reference) && (*REFERENCE.read().unwrap() > 0 || no_limits) {
            *REFERENCE.write().unwrap() -= 1;
            collected.push(Reference::arbitrary().prop_map(Element::Reference).boxed());
        }
        if targets.contains(&ElTarget::Task) && (*TASK.read().unwrap() > 0 || no_limits) {
            *TASK.write().unwrap() -= 1;
            collected.push(Task::arbitrary().prop_map(Element::Task).boxed());
        }
        if targets.contains(&ElTarget::Values) && (*VALUES.read().unwrap() > 0 || no_limits) {
            *VALUES.write().unwrap() -= 1;
            collected.push(Values::arbitrary().prop_map(Element::Values).boxed());
        }
        if targets.contains(&ElTarget::VariableAssignation)
            && (*VARIABLE_ASSIGNATION.read().unwrap() > 0 || no_limits)
        {
            *VARIABLE_ASSIGNATION.write().unwrap() -= 1;
            collected.push(
                VariableAssignation::arbitrary()
                    .prop_map(Element::VariableAssignation)
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableName)
            && (*VARIABLE_NAME.read().unwrap() > 0 || no_limits)
        {
            *VARIABLE_NAME.write().unwrap() -= 1;
            collected.push(
                VariableName::arbitrary()
                    .prop_map(Element::VariableName)
                    .boxed(),
            );
        }
        collected
    }

    impl Arbitrary for Element {
        type Parameters = Vec<ElTarget>;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(targets: Self::Parameters) -> Self::Strategy {
            let mut collected = generate(&targets, false);
            if collected.is_empty() {
                collected = generate(&targets, true);
            }
            prop::strategy::Union::new(collected).boxed()
        }
    }

    fn reading(el: Element) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("{el};");
            let mut reader = Reader::unbound(origin.clone());
            while let Some(block) = Block::read(&mut reader)? {
                assert_eq!(format!("{block};"), origin);
            }
            Ok(())
        })
    }

    // proptest! {
    //     #![proptest_config(ProptestConfig {
    //         max_shrink_iters: 5000,
    //         ..ProptestConfig::with_cases(10)
    //     })]
    //     #[test]
    //     fn test_run_task(
    //         args in any_with::<Element>(vec![ElTarget::Function])
    //     ) {
    //         let res = reading(args.clone());
    //         if res.is_err() {
    //             println!("{res:?}");
    //         }
    //         prop_assert!(res.is_ok());
    //     }
    // }
}
