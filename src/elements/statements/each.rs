use crate::{
    elements::{Block, Component, ElTarget, Element, VariableName},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;
#[derive(Debug, Clone)]
pub struct Each {
    pub variable: VariableName,
    pub input: Box<Element>,
    pub block: Block,
    pub token: usize,
}

impl Reading<Each> for Each {
    fn read(reader: &mut Reader) -> Result<Option<Each>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader.move_to().word(&[words::EACH]).is_some() {
            let (variable, input) = if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let variable = if let Some(Element::VariableName(variable, _)) =
                    Element::include(&mut inner, &[ElTarget::VariableName])?
                {
                    if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return Err(E::MissedSemicolon.linked(&inner.token()?.id));
                    }
                    variable
                } else {
                    return Err(E::NoLoopVariable.linked(&inner.token()?.id));
                };
                let input = if let Some(el) =
                    Element::include(&mut inner, &[ElTarget::Function, ElTarget::VariableName])?
                {
                    Box::new(el)
                } else {
                    Err(E::NoLoopInput.by_reader(&inner))?
                };
                (variable, input)
            } else {
                return Err(E::NoLoopInitialization.linked(&reader.token()?.id));
            };
            let block = if let Some(Element::Block(block, _)) =
                Element::include(reader, &[ElTarget::Block])?
            {
                block
            } else {
                Err(E::NoGroup.by_reader(reader))?
            };
            Ok(Some(Each {
                input,
                variable,
                block,
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Each {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EACH({}; {}) {}", self.variable, self.input, self.block)
    }
}

impl Formation for Each {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}EACH({}; {}) {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.variable,
            self.input,
            self.block.format(cursor)
        )
    }
}

impl Operator for Each {
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
            let inputs = self
                .input
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoInputForEach)?
                .get_as_strings()
                .ok_or(operator::E::FailConvertInputIntoStringsForEach)?;
            let mut output: Option<AnyValue> = None;
            for iteration in inputs.iter() {
                cx.vars().set(
                    self.variable.name.to_owned(),
                    AnyValue::new(iteration.to_string()),
                );
                output = self.block.execute(owner, components, args, cx).await?;
            }
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
        elements::Each,
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests::*},
        reader::{chars, Reader, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../../tests/reading/each.sibs"));
        let mut count = 0;
        while let Some(entity) = report_if_err(&cx, Each::read(&mut reader))? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                trim_carets(reader.recent()),
                trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 7);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../../tests/reading/each.sibs"));
        let mut count = 0;
        while let Some(entity) = Each::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                trim_carets(&format!("{entity}")),
                trim_carets(&reader.get_fragment(&entity.token)?.lined),
            );
            assert_eq!(
                trim_carets(&entity.block.to_string()),
                trim_carets(&reader.get_fragment(&entity.block.token)?.lined),
            );
            assert_eq!(
                trim_carets(&entity.variable.to_string()),
                trim_carets(&reader.get_fragment(&entity.variable.token)?.lined),
            );
            assert_eq!(
                trim_carets(&entity.input.to_string()),
                trim_carets(&reader.get_fragment(&entity.input.token())?.lined),
            );
            count += 1;
        }
        assert_eq!(count, 7);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn error() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let samples = include_str!("../../tests/error/each.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = cx.reader().from_str(sample);
            assert!(Each::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{chars, Reading},
    };
    const VALUES: &[(&str, &str)] = &[("a", "three"), ("b", "two"), ("c", "one")];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../../tests/processing/each.sibs"));
        while let Some(task) = Task::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert!(task.execute(None, &[], &[], &mut cx).await?.is_some());
        }
        for (name, value) in VALUES.iter() {
            assert_eq!(
                cx.vars().get(name).unwrap().get_as_string().unwrap(),
                value.to_string()
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{task::Task, Block, Each, ElTarget, Element, VariableName},
        inf::{operator::E, tests::*, Context},
        reader::Reading,
    };
    use proptest::prelude::*;

    impl Arbitrary for Each {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Block::arbitrary_with(deep),
                VariableName::arbitrary(),
                Element::arbitrary_with((vec![ElTarget::VariableName], deep)),
            )
                .prop_map(|(block, variable, input)| Each {
                    block,
                    variable,
                    input: Box::new(input),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(each: Each) -> Result<(), E> {
        get_rt().block_on(async {
            let mut cx = Context::create().unbound()?;
            let origin = format!("test [\n{each};\n];");
            let mut reader = cx.reader().from_str(&origin);
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
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
            args in any_with::<Each>(0)
        ) {
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
