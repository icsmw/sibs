use crate::{
    entry::{Block, Cmp, Component, ElTarget, Element, Function, PatternString, VariableName},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::{fmt, ops::Deref};

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

// #[derive(Debug, Clone)]
// pub enum CmpSubject {
//     VariableName(VariableName),
//     PatternString(PatternString),
//     Function(Function),
// }

// impl CmpSubject {
//     pub fn read(reader: &mut Reader) -> Result<Self, LinkedErr<E>> {
//         let subject = if let Some(variable) = VariableName::read(reader)? {
//             Self::VariableName(variable)
//         } else if let Some(pattern) = PatternString::read(reader)? {
//             Self::PatternString(pattern)
//         } else if let Some(func) = Function::read(reader)? {
//             Self::Function(func)
//         } else {
//             return Err(E::NotSupportedInputForIF.linked(&reader.token()?.id));
//         };
//         if !reader.rest().trim().is_empty() {
//             Err(E::FailToParseSideOfComparing.linked(&reader.token()?.id))
//         } else {
//             Ok(subject)
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum Proviso {
    // bool -          true  - negative;
    //                 false - positive;
    Condition(Element, bool),
    Combination(Combination, usize),
    // bool: true - if root level; false - if nested group
    Group(bool, Vec<Proviso>, usize),
}

impl Operator for Proviso {
    fn token(&self) -> usize {
        match self {
            Self::Condition(el, _) => el.token(),
            Self::Combination(_, token) => *token,
            Self::Group(_, _, token) => *token,
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
                Self::Condition(el, negative) => {
                    let result = *match el {
                        Element::Comparing(el) => el.execute(owner, components, args, cx).await?,
                        Element::Function(el) => el.execute(owner, components, args, cx).await?,
                        _ => {
                            return Err(operator::E::UnsupportedCondition);
                        }
                    }
                    .ok_or(operator::E::FailExtractValueForIFStatement)?
                    .get_as::<bool>()
                    .ok_or(operator::E::NoBoolResultFromFunction)?;
                    Ok(Some(AnyValue::new(if *negative {
                        !result
                    } else {
                        result
                    })))
                }

                Self::Group(_, provisos, _) => {
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
                                    .execute(owner, components, args, cx)
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
                Self::Condition(el, negative) =>
                    format!("{}{el}", if *negative { "!" } else { "" }),
                Self::Combination(v, _) => v.to_string(),
                Self::Group(root, provisio, _) => {
                    format!(
                        "{}{}{}",
                        if *root { "" } else { "(" },
                        provisio
                            .iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<String>>()
                            .join(" "),
                        if *root { "" } else { ")" }
                    )
                }
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Segment {
    If(Proviso, Block),
    Else(Block),
}

impl Operator for Segment {
    fn token(&self) -> usize {
        match self {
            Self::If(proviso, _) => proviso.token(),
            Self::Else(block) => block.token(),
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
                Self::If(proviso, block) => {
                    if *proviso
                        .execute(owner, components, args, cx)
                        .await?
                        .ok_or(operator::E::NoResultFromProviso)?
                        .get_as::<bool>()
                        .ok_or(operator::E::NoBoolResultFromProviso)?
                    {
                        block.execute(owner, components, args, cx).await
                    } else {
                        Ok(None)
                    }
                }
                Self::Else(block) => block.execute(owner, components, args, cx).await,
            }
        })
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Segment::If(provisio, block) => format!("IF {provisio} {block}"),
                Segment::Else(block) => format!("ELSE {block}"),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub elements: Vec<Segment>,
    pub token: usize,
}

impl Reading<If> for If {
    fn read(reader: &mut Reader) -> Result<Option<If>, LinkedErr<E>> {
        let mut elements: Vec<Segment> = vec![];
        let close = reader.open_token();
        while !reader.rest().trim().is_empty() {
            if reader.move_to().word(&[words::IF]).is_some() {
                if reader.until().char(&[&chars::OPEN_SQ_BRACKET]).is_some() {
                    let proviso: Proviso = If::proviso(&mut reader.token()?.bound, true)?;
                    if let Some(block) = Block::read(reader)? {
                        elements.push(Segment::If(proviso, block));
                    } else {
                        Err(E::NotClosedGroup.linked(&proviso.token()))?
                    }
                    continue;
                } else {
                    Err(E::NoGroup.by_reader(reader))?
                }
            }
            if elements.is_empty() {
                return Ok(None);
            }
            if reader.move_to().char(&[&chars::SEMICOLON]).is_some() {
                return Ok(Some(If {
                    elements,
                    token: close(reader),
                }));
            }
            if reader.move_to().word(&[words::ELSE]).is_some() {
                if let Some(block) = Block::read(reader)? {
                    elements.push(Segment::Else(block));
                    if reader.move_to().char(&[&chars::SEMICOLON]).is_some() {
                        return Ok(Some(If {
                            elements,
                            token: close(reader),
                        }));
                    } else {
                        Err(E::MissedSemicolon.by_reader(reader))?
                    }
                } else {
                    Err(E::NoGroup.by_reader(reader))?
                }
            } else {
                Err(E::MissedSemicolon.by_reader(reader))?
            }
        }
        Ok(None)
    }
}

impl If {
    pub fn inner(reader: &mut Reader) -> Result<Proviso, LinkedErr<E>> {
        let negative = reader.move_to().char(&[&chars::EXCLAMATION]).is_some();
        if let Some(el) = Element::include(reader, &[ElTarget::Comparing, ElTarget::Function])? {
            Ok(Proviso::Condition(el, negative))
        } else {
            Err(E::NoProvisoOfCondition.by_reader(&reader))
        }
    }
    pub fn proviso(reader: &mut Reader, root: bool) -> Result<Proviso, LinkedErr<E>> {
        let mut proviso: Vec<Proviso> = vec![];
        let close = reader.open_token();
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
                        Err(E::NestedConditionGroups.by_reader(reader))?
                    }
                    proviso.push(If::proviso(&mut group_reader, false)?);
                    continue;
                } else {
                    Err(E::NotClosedConditionGroup.by_reader(reader))?
                }
            }
            if let Some((_, combination)) = reader.until().word(&[words::AND, words::OR]) {
                let mut token = reader.token()?;
                if !reader.move_to().whitespace() {
                    Err(E::NoWhitespaceAfterCondition.by_reader(reader))?;
                }
                if proviso.last().is_none() && token.content.trim().is_empty() {
                    Err(E::NoProvisoOfCondition.by_reader(reader))?
                }
                if !token.content.trim().is_empty() {
                    proviso.push(If::inner(&mut token.bound)?);
                }
                if let Some(Proviso::Combination(_, _)) = proviso.last() {
                    Err(E::RepeatedCombinationOperator.by_reader(reader))?
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
                Err(E::NoProvisoOfCondition.by_reader(reader))?
            }
        }
        Ok(Proviso::Group(root, proviso, close(reader)))
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                .map(|el| el.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Operator for If {
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
            for element in self.elements.iter() {
                if let Some(output) = element.execute(owner, components, args, cx).await? {
                    return Ok(Some(output));
                }
            }
            Ok(None)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::{statements::If::Segment, If},
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(include_str!("../../tests/reading/if.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = If::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 10);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(include_str!("../../tests/reading/if.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = If::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&format!("{entity};")),
                tests::trim_carets(&reader.get_fragment(&entity.token)?.lined)
            );
            for el in entity.elements.iter() {
                match el {
                    Segment::If(proviso, block) => {
                        assert_eq!(
                            tests::trim_carets(&proviso.to_string()),
                            tests::trim_carets(&reader.get_fragment(&proviso.token())?.lined)
                        );
                        assert_eq!(
                            tests::trim_carets(&block.to_string()),
                            tests::trim_carets(&reader.get_fragment(&block.token)?.lined)
                        );
                    }
                    Segment::Else(block) => {
                        assert_eq!(
                            tests::trim_carets(&block.to_string()),
                            tests::trim_carets(&reader.get_fragment(&block.token)?.lined)
                        );
                    }
                };
            }
            count += 1;
        }
        assert_eq!(count, 10);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/if.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
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
        entry::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{Reader, Reading},
    };

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../../tests/processing/if.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(None, &[], &[], &mut cx)
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
        entry::{
            comparing::Cmp,
            element::Element,
            statements::If::{Combination, If, Proviso, Segment},
            task::Task,
            Block,
        },
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

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
                Element::arbitrary_with(scope.clone())
                    .prop_map(|cmb| Proviso::Condition(cmb, false)),
                max,
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

    // Returns Proviso::Group
    fn get_proviso(scope: SharedScope) -> BoxedStrategy<Proviso> {
        let max = 5;
        prop::collection::vec(
            (
                get_proviso_chain(scope.clone()),
                Combination::arbitrary_with(scope.clone())
                    .prop_map(|cmb| Proviso::Combination(cmb, 0)),
                prop_oneof![Just(true), Just(false)],
            ),
            1..max,
        )
        .prop_map(|mut combinations| {
            let mut provisos: Vec<Proviso> = vec![];
            while let Some((chain, combination, grouping)) = combinations.pop() {
                if chain.is_empty() {
                    continue;
                }
                if !provisos.is_empty() {
                    provisos.push(combination);
                }
                if grouping {
                    provisos.push(Proviso::Group(false, chain, 0));
                } else {
                    provisos = [provisos, chain].concat();
                }
            }
            Proviso::Group(true, provisos, 0)
        })
        .boxed()
    }

    impl Arbitrary for Segment {
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
                    .prop_map(|(p, b)| Segment::If(p, b))
                    .boxed(),
                _ => Block::arbitrary_with(scope.clone())
                    .prop_map(Segment::Else)
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
                prop::collection::vec(Segment::arbitrary_with((0, scope.clone())), 1..5),
                prop::collection::vec(Segment::arbitrary_with((1, scope.clone())), 0..1),
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

    fn reading(if_block: If) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("test [\n{if_block};\n];");
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
            args in any_with::<If>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
