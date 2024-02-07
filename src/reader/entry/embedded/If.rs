use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Block, Component, Function, Reading, ValueString, VariableName},
        words, Reader, E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Cmp {
    Equal,
    NotEqual,
}

impl fmt::Display for Cmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equal => words::CMP_TRUE,
                Self::NotEqual => words::CMP_FALSE,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Combination {
    And,
    Or,
}

impl fmt::Display for Combination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => words::AND,
                Self::Or => words::OR,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Proviso {
    Variable(VariableName, Cmp, ValueString),
    // Function, is negative (!)
    Function(Function, bool),
    Combination(Combination, usize),
    Group(Vec<Proviso>),
}

impl Operator for Proviso {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::Variable(name, cmp, value) => {
                    let left = name
                        .process(owner, components, args, cx)
                        .await?
                        .ok_or(operator::E::VariableIsNotAssigned(name.name.clone()))?
                        .get_as_string()
                        .ok_or(operator::E::FailToGetValueAsString)?;
                    let right = value
                        .process(owner, components, args, cx)
                        .await?
                        .ok_or(operator::E::FailToGetStringValue)?
                        .get_as_string()
                        .ok_or(operator::E::FailToGetValueAsString)?;
                    Ok(Some(AnyValue::new(match cmp.clone() {
                        Cmp::Equal => left == right,
                        Cmp::NotEqual => left != right,
                    })))
                }
                Self::Function(func, negative) => {
                    let result = *func
                        .process(owner, components, args, cx)
                        .await?
                        .ok_or(operator::E::NoBoolResultFromFunction)?
                        .get_as::<bool>()
                        .ok_or(operator::E::NoBoolResultFromFunction)?;
                    Ok(Some(AnyValue::new(if *negative {
                        !result
                    } else {
                        result
                    })))
                }
                Self::Group(provisos) => {
                    let mut iteration: Option<bool> = None;
                    for proviso in provisos.iter() {
                        if let Proviso::Combination(cmb, _) = proviso {
                            if iteration.is_none() {
                                Err(operator::E::WrongConditionsOrderInIf)?;
                            }
                            if let (true, Some(true)) = (matches!(cmb, Combination::Or), iteration)
                            {
                                return Ok(Some(AnyValue::new(true)));
                            } else if let (true, Some(false)) =
                                (matches!(cmb, Combination::And), iteration)
                            {
                                return Ok(Some(AnyValue::new(false)));
                            }
                        } else {
                            iteration = Some(
                                *proviso
                                    .process(owner, components, args, cx)
                                    .await?
                                    .ok_or(operator::E::NoResultFromProviso)?
                                    .get_as::<bool>()
                                    .ok_or(operator::E::NoBoolResultFromProviso)?,
                            );
                        }
                    }
                    Ok(Some(AnyValue::new(
                        iteration.ok_or(operator::E::NoBoolResultFromProvisoGroup)?,
                    )))
                }
                Self::Combination(_, _) => Err(operator::E::WrongConditionsOrderInIf),
            }
        })
    }
}

impl fmt::Display for Proviso {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Variable(variable_name, cmp, value_string) =>
                    format!("{variable_name} {cmp} {value_string}"),
                Self::Combination(v, _) => v.to_string(),
                Self::Function(v, negative) => format!("{}{v}", if *negative { "!" } else { "" }),
                Self::Group(provisio) => provisio
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    If(Proviso, Block),
    Else(Block),
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
                Self::If(proviso, block) => {
                    if *proviso
                        .process(owner, components, args, cx)
                        .await?
                        .ok_or(operator::E::NoResultFromProviso)?
                        .get_as::<bool>()
                        .ok_or(operator::E::NoBoolResultFromProviso)?
                    {
                        block.process(owner, components, args, cx).await
                    } else {
                        Ok(None)
                    }
                }
                Self::Else(block) => block.process(owner, components, args, cx).await,
            }
        })
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::If(provisio, block) => format!("IF {provisio} {block}"),
                Element::Else(block) => format!("ELSE {block}"),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub elements: Vec<Element>,
    pub token: usize,
}

impl Reading<If> for If {
    fn read(reader: &mut Reader) -> Result<Option<If>, E> {
        let mut elements: Vec<Element> = vec![];
        while !reader.rest().trim().is_empty() {
            if reader.move_to().word(&[&words::IF]).is_some() {
                if reader.until().char(&[&chars::OPEN_SQ_BRACKET]).is_some() {
                    let proviso: Proviso = If::proviso(&mut reader.token()?.bound)?;
                    if reader
                        .group()
                        .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                        .is_some()
                    {
                        if let Some(block) = Block::read(&mut reader.token()?.bound)? {
                            elements.push(Element::If(proviso, block));
                        } else {
                            Err(E::EmptyGroup)?
                        }
                    } else {
                        Err(E::NotClosedGroup)?
                    }
                    continue;
                } else {
                    Err(E::NoGroup)?
                }
            }
            if elements.is_empty() {
                return Ok(None);
            }
            if reader.move_to().char(&[&chars::SEMICOLON]).is_some() {
                return Ok(Some(If {
                    elements,
                    token: reader.token()?.id,
                }));
            }
            if reader.move_to().word(&[&words::ELSE]).is_some() {
                if reader
                    .group()
                    .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                    .is_some()
                {
                    if let Some(block) = Block::read(&mut reader.token()?.bound)? {
                        elements.push(Element::Else(block));
                    } else {
                        Err(E::EmptyGroup)?
                    }
                    if reader.move_to().char(&[&chars::SEMICOLON]).is_some() {
                        return Ok(Some(If {
                            elements,
                            token: reader.token()?.id,
                        }));
                    } else {
                        Err(E::MissedSemicolon)?
                    }
                } else {
                    Err(E::NoGroup)?
                }
            } else {
                Err(E::MissedSemicolon)?
            }
        }
        Ok(None)
    }
}

impl If {
    pub fn inner(reader: &mut Reader) -> Result<Proviso, E> {
        if let Some(variable_name) = VariableName::read(reader)? {
            if let Some(word) = reader
                .move_to()
                .word(&[&words::CMP_TRUE, &words::CMP_FALSE])
            {
                if let Some(value_string) = ValueString::read(reader)? {
                    return Ok(Proviso::Variable(
                        variable_name,
                        if word == words::CMP_TRUE {
                            Cmp::Equal
                        } else {
                            Cmp::NotEqual
                        },
                        value_string,
                    ));
                } else {
                    Err(E::NoStringValueWithCondition)?
                }
            } else {
                Err(E::MissedComparingOperator)?
            }
        }
        let negative = reader.move_to().char(&[&chars::EXCLAMATION]).is_some();
        if let Some(func) = Function::read(reader)? {
            Ok(Proviso::Function(func, negative))
        } else {
            Err(E::NoProvisoOfCondition)
        }
    }
    pub fn proviso(reader: &mut Reader) -> Result<Proviso, E> {
        let mut proviso: Vec<Proviso> = vec![];
        while !reader.rest().trim().is_empty() {
            if reader.move_to().char(&[&chars::OPEN_BRACKET]).is_some() {
                if reader.until().char(&[&chars::CLOSE_BRACKET]).is_some() {
                    let mut group_reader = reader.token()?.bound;
                    let _ = reader.move_to().next();
                    if group_reader
                        .move_to()
                        .char(&[&chars::OPEN_BRACKET])
                        .is_some()
                    {
                        Err(E::NestedConditionGroups)?
                    }
                    proviso.push(If::proviso(&mut group_reader)?);
                    continue;
                } else {
                    Err(E::NotClosedConditionGroup)?
                }
            }
            if let Some((_, combination)) = reader.until().word(&[&words::AND, &words::OR]) {
                let mut token = reader.token()?;
                if !reader.move_to().whitespace() {
                    Err(E::NoWhitespaceAfterCondition)?;
                }
                if proviso.last().is_none() && token.content.trim().is_empty() {
                    Err(E::NoProvisoOfCondition)?
                }
                if !token.content.trim().is_empty() {
                    proviso.push(If::inner(&mut token.bound)?);
                }
                if let Some(Proviso::Combination(_, _)) = proviso.last() {
                    Err(E::RepeatedCombinationOperator)?
                }
                proviso.push(Proviso::Combination(
                    if combination == words::AND {
                        Combination::And
                    } else {
                        Combination::Or
                    },
                    token.id,
                ));
                continue;
            }
            if matches!(proviso.last(), Some(Proviso::Combination(_, _))) || proviso.is_empty() {
                proviso.push(If::inner(reader)?);
            } else {
                Err(E::NoProvisoOfCondition)?
            }
        }
        Ok(Proviso::Group(proviso))
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{};",
            self.elements
                .iter()
                .map(|el| el.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Operator for If {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            for element in self.elements.iter() {
                if let Some(output) = element.process(owner, components, args, cx).await? {
                    return Ok(Some(output));
                }
            }
            Ok(None)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::reader::{
        entry::{If, Reading},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("../../../tests/reading/if.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = If::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 10);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../../tests/error/if.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(If::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{
            entry::{Reading, Task},
            Reader,
        },
    };

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader = Reader::new(include_str!("../../../tests/processing/if.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .process(None, &[], &[], &mut cx)
                .await?
                .expect("IF returns some value");
            assert_eq!(
                result.get_as_string().expect("IF returns string value"),
                "true".to_owned()
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        inf::tests::*,
        reader::entry::{
            embedded::If::{Cmp, Combination, Element, If, Proviso},
            function::Function,
            value_strings::ValueString,
            variable_name::VariableName,
            Block,
        },
    };
    use proptest::prelude::*;

    impl Arbitrary for Cmp {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_scope: Self::Parameters) -> Self::Strategy {
            prop_oneof![Just(Cmp::Equal), Just(Cmp::NotEqual),].boxed()
        }
    }

    impl Arbitrary for Combination {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_scope: Self::Parameters) -> Self::Strategy {
            prop_oneof![Just(Combination::And), Just(Combination::Or),].boxed()
        }
    }

    // Gives flat Proviso with:
    // - Combination
    // - Variable
    // - Function
    fn get_proviso_chain(scope: SharedScope) -> BoxedStrategy<Vec<Proviso>> {
        let max = 6;
        (
            prop::collection::vec(
                Combination::arbitrary_with(scope.clone())
                    .prop_map(|cmb| Proviso::Combination(cmb, 0)),
                max,
            ),
            prop::collection::vec(
                {
                    let permissions = scope.read().unwrap().permissions();
                    let mut allowed = vec![(
                        VariableName::arbitrary(),
                        Cmp::arbitrary_with(scope.clone()),
                        ValueString::arbitrary_primitive(scope.clone()),
                    )
                        .prop_map(|(name, cmp, value)| Proviso::Variable(name, cmp, value))
                        .boxed()];
                    if permissions.func {
                        allowed.push(
                            Function::arbitrary_with(scope.clone())
                                .prop_map(|f| Proviso::Function(f, false))
                                .boxed(),
                        );
                    }
                    if permissions.value_string {
                        allowed.push(
                            (
                                VariableName::arbitrary(),
                                Cmp::arbitrary_with(scope.clone()),
                                ValueString::arbitrary_with(scope.clone()),
                            )
                                .prop_map(|(name, cmp, value)| Proviso::Variable(name, cmp, value))
                                .boxed(),
                        );
                    }
                    prop::strategy::Union::new(allowed).boxed()
                },
                1..max,
            ),
        )
            .prop_map(|(mut combinations, conditions)| {
                let mut provisos: Vec<Proviso> = conditions
                    .into_iter()
                    .flat_map(|condition| [condition, combinations.remove(0)])
                    .collect();
                if let Some(Proviso::Combination(_, _)) = provisos.last() {
                    let _ = provisos.pop();
                }
                provisos
            })
            .boxed()
    }

    // Creates proviso group with nested groups
    fn get_proviso_group(scope: SharedScope) -> BoxedStrategy<Proviso> {
        let max = 5;
        (
            prop::collection::vec(
                Combination::arbitrary_with(scope.clone())
                    .prop_map(|cmb| Proviso::Combination(cmb, 0)),
                max,
            ),
            prop::collection::vec(get_proviso_chain(scope.clone()), max),
        )
            .prop_map(|(mut combinations, mut chains)| {
                while chains.len() >= 2 {
                    let group = Proviso::Group(chains.remove(0));
                    chains
                        .get_mut(0)
                        .unwrap()
                        .splice(0..0, vec![combinations.remove(0), group]);
                }
                Proviso::Group(chains.remove(0))
            })
            .boxed()
    }

    // Returns Proviso::Group
    fn get_proviso(scope: SharedScope) -> BoxedStrategy<Proviso> {
        let max = 5;
        (
            prop::collection::vec(
                Combination::arbitrary_with(scope.clone())
                    .prop_map(|cmb| Proviso::Combination(cmb, 0)),
                max,
            ),
            prop::collection::vec(get_proviso_group(scope.clone()), 1..max),
        )
            .prop_map(|(mut combinations, groups)| {
                let mut provisos: Vec<Proviso> = groups
                    .into_iter()
                    .flat_map(|group| [group, combinations.remove(0)])
                    .collect();
                if let Some(Proviso::Combination(_, _)) = provisos.last() {
                    let _ = provisos.pop();
                }
                Proviso::Group(provisos)
            })
            .boxed()
    }

    impl Arbitrary for Element {
        /// 0 - generate IF
        /// 1 - generate ELSE
        type Parameters = (u8, SharedScope);
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with((el, scope): Self::Parameters) -> Self::Strategy {
            match el {
                0 => (
                    get_proviso(scope.clone()),
                    Block::arbitrary_with(scope.clone()),
                )
                    .prop_map(|(p, b)| Element::If(p, b))
                    .boxed(),
                _ => Block::arbitrary_with(scope.clone())
                    .prop_map(Element::Else)
                    .boxed(),
            }
        }
    }

    impl Arbitrary for If {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::If);
            let boxed = (
                prop::collection::vec(Element::arbitrary_with((0, scope.clone())), 1..5),
                prop::collection::vec(Element::arbitrary_with((1, scope.clone())), 0..1),
            )
                .prop_map(|(elements, else_element)| If {
                    elements: [elements, else_element].concat(),
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::If);
            boxed
        }
    }
}
