use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{operator, Context, Formation, FormationCursor, Operator, OperatorPinnedResult},
    reader::{words, Reader, Reading, E},
};
use std::fmt;
use tokio::join;

#[derive(Debug, Clone)]
pub struct Join {
    pub elements: Box<Element>,
    pub token: usize,
}

impl Reading<Join> for Join {
    fn read(reader: &mut Reader) -> Result<Option<Join>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader.move_to().word(&[words::JOIN]).is_some() {
            let Some(Element::Values(elements, md)) =
                Element::include(reader, &[ElTarget::Values])?
            else {
                return Err(E::NoJOINStatementBody.by_reader(reader));
            };
            if elements.elements.is_empty() {
                Err(E::NoJOINStatementBody.by_reader(reader))?;
            }
            for el in elements.elements.iter() {
                if !matches!(
                    el,
                    Element::Reference(..) | Element::Function(..) | Element::Command(..)
                ) {
                    Err(E::NotReferenceInJOIN.linked(&el.token()))?;
                }
            }
            Ok(Some(Join {
                elements: Box::new(Element::Values(elements, md)),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JOIN {}", self.elements)
    }
}

impl Formation for Join {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::Join));
        format!(
            "{}JOIN {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.elements.format(&mut inner)
        )
    }
}

impl Operator for Join {
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
            // let Element::Values(values, _) = self.elements.as_ref() else {
            //     return Err(operator::E::NoOperationsToJoin.by(self));
            // };
            // let mut operations: Vec<OperatorPinnedResult> = vec![];
            // for el in values.elements.iter() {
            //     operations.push(el.execute(owner, components, args, cx));
            // }
            // let operations = values
            //     .elements
            //     .iter()
            //     .map(|o| o.execute(owner, components, args, cx))
            //     .collect::<Vec<OperatorPinnedResult>>();
            Ok(None)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Join,
        error::LinkedErr,
        inf::{tests::*, Operator},
        reader::{chars, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        runner(
            include_str!("../../tests/reading/join.sibs"),
            |mut src, mut reader| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Join::read(&mut reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 2);
                assert!(reader.rest().trim().is_empty());
                Ok(())
            },
        )
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        runner(
            include_str!("../../tests/reading/join.sibs"),
            |_, mut reader| {
                let mut count = 0;
                while let Some(entity) = Join::read(&mut reader)? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.elements.to_string()),
                        trim_carets(&reader.get_fragment(&entity.elements.token())?.lined),
                    );
                    count += 1;
                }
                assert_eq!(count, 2);
                assert!(reader.rest().trim().is_empty());
                Ok(())
            },
        )
    }

    #[tokio::test]
    async fn error() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../../tests/error/join.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += runner(sample, |_, mut reader| {
                assert!(Join::read(&mut reader).is_err());
                Ok(1)
            })?;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{ElTarget, Element, Join, Metadata, Task, Values},
        error::LinkedErr,
        inf::{operator::E, tests::*},
        reader::Reading,
    };
    use proptest::prelude::*;

    impl Arbitrary for Join {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            prop::collection::vec(
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElTarget::Reference]
                    } else {
                        vec![ElTarget::Reference, ElTarget::Function, ElTarget::Command]
                    },
                    deep,
                )),
                1..=10,
            )
            .prop_map(|elements| Values { elements, token: 0 })
            .prop_map(|elements| Join {
                elements: Box::new(Element::Values(elements, Metadata::empty())),
                token: 0,
            })
            .boxed()
        }
    }

    fn reading(join: Join) -> Result<(), LinkedErr<E>> {
        get_rt().block_on(async {
            let origin = format!("test [\n{join};\n];");
            runner(&origin, |_, mut reader| {
                while let Some(task) = Task::read(&mut reader)? {
                    assert_eq!(format!("{task};"), origin);
                }
                Ok::<(), LinkedErr<E>>(())
            })?;
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
            args in any_with::<Join>(0)
        ) {
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
