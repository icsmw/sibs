use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
        OperatorResult, Scope,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use futures::stream::{FuturesUnordered, StreamExt};
use std::fmt;
use tokio::{
    spawn,
    task::{JoinError, JoinHandle},
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Join {
    pub elements: Box<Element>,
    pub token: usize,
}

impl TryDissect<Join> for Join {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Join>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Join);
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

impl Dissect<Join, Join> for Join {}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "join {}", self.elements)
    }
}

impl Formation for Join {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::Join));
        format!(
            "{}join {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.elements.format(&mut inner)
        )
    }
}

enum TaskError {
    Join(JoinError),
}

impl From<TaskError> for LinkedErr<operator::E> {
    fn from(err: TaskError) -> Self {
        match err {
            TaskError::Join(err) => operator::E::JoinError(err.to_string()).unlinked(),
        }
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
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> OperatorPinnedResult {
        fn clone(
            owner: Option<&Component>,
            components: &[Component],
            args: &[String],
            cx: &Context,
            sc: &Scope,
            token: &CancellationToken,
        ) -> (
            Option<Component>,
            Vec<Component>,
            Vec<String>,
            Context,
            Scope,
            CancellationToken,
        ) {
            (
                owner.cloned().clone(),
                components.to_vec(),
                args.to_vec(),
                cx.clone(),
                sc.clone(),
                token.clone(),
            )
        }
        async fn wait(
            tasks: &mut [JoinHandle<OperatorResult>],
            token: CancellationToken,
        ) -> Result<Vec<Result<AnyValue, LinkedErr<operator::E>>>, TaskError> {
            let mut results: Vec<Result<AnyValue, LinkedErr<operator::E>>> = Vec::new();
            let mut futures = FuturesUnordered::new();
            for task in tasks {
                futures.push(task);
            }
            while let Some(result) = futures.next().await {
                match result {
                    Ok(Ok(result)) => {
                        results.push(Ok(result.unwrap_or_else(AnyValue::empty)));
                    }
                    Ok(Err(err)) => {
                        if !token.is_cancelled() {
                            token.cancel();
                        }
                        results.push(Err(err));
                        // return Err(TaskError::Operator(err));
                    }
                    Err(err) => {
                        return Err(TaskError::Join(err));
                    }
                }
            }
            Ok(results)
        }
        Box::pin(async move {
            let Element::Values(values, _) = self.elements.as_ref() else {
                return Ok(None);
            };
            let mut tasks = values
                .elements
                .iter()
                .cloned()
                .map(|el| {
                    let params = clone(owner, components, args, &cx, &sc, &token);
                    spawn(async move {
                        // inside exclude will be create clone
                        el.execute(
                            params.0.as_ref(),
                            &params.1,
                            &params.2,
                            params.3,
                            params.4,
                            params.5,
                        )
                        .await
                    })
                })
                .collect::<Vec<JoinHandle<OperatorResult>>>();
            match wait(&mut tasks, token).await {
                Ok(results) => {
                    let mut output: Vec<AnyValue> = Vec::new();
                    for result in results.into_iter() {
                        match result {
                            Ok(value) => {
                                output.push(value);
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        };
                    }
                    Ok(Some(AnyValue::new(output)?))
                }
                Err(err) => Err(err.into()),
            }
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Join,
        error::LinkedErr,
        inf::{tests::*, Configuration, Operator},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/join.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Join::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 2);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/join.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Join::dissect(reader))? {
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
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/join.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(Join::dissect(reader).is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::Element,
        error::LinkedErr,
        inf::{
            journal::{Configuration, Journal},
            AnyValue, Context, Operator, Scope,
        },
        process_file,
        reader::{error::E, Reader, Sources},
    };

    #[tokio::test]
    async fn reading() {
        let target = std::env::current_dir()
            .unwrap()
            .join("./src/tests/processing/join.sibs");
        process_file!(
            &Configuration::logs(false),
            &target,
            |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                assert_eq!(elements.len(), 1);
                let Some(Element::Component(el, _md)) = elements.first() else {
                    panic!("Component isn't found");
                };
                let results = el
                    .execute(
                        None,
                        &[],
                        &[String::from("test_a")],
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new(),
                    )
                    .await
                    .expect("run is successfull")
                    .expect("join returns vector of results");
                assert!(results.get::<Vec<AnyValue>>().is_some());
                assert_eq!(
                    results
                        .get::<Vec<AnyValue>>()
                        .expect("join returns Vec<AnyValue>")
                        .len(),
                    4
                );
                let results = el
                    .execute(
                        Some(el),
                        &[],
                        &[String::from("test_b")],
                        cx,
                        sc,
                        CancellationToken::new(),
                    )
                    .await
                    .expect("run is successfull")
                    .expect("join returns vector of results");
                assert!(results.get::<Vec<AnyValue>>().is_some());
                assert_eq!(
                    results
                        .get::<Vec<AnyValue>>()
                        .expect("join returns Vec<AnyValue>")
                        .len(),
                    2
                );
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn errors() {
        let target = std::env::current_dir()
            .unwrap()
            .join("./src/tests/processing/join.sibs");
        process_file!(
            &Configuration::logs(false),
            &target,
            |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                assert_eq!(elements.len(), 1);
                let Some(Element::Component(el, _md)) = elements.first() else {
                    panic!("Component isn't found");
                };
                assert!(el
                    .execute(
                        Some(el),
                        &[],
                        &[String::from("test_c")],
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new()
                    )
                    .await
                    .is_err());
                assert!(el
                    .execute(
                        Some(el),
                        &[],
                        &[String::from("test_d")],
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new()
                    )
                    .await
                    .is_err());
                assert!(el
                    .execute(
                        Some(el),
                        &[],
                        &[String::from("test_e")],
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new()
                    )
                    .await
                    .is_err());
                assert!(el
                    .execute(
                        Some(el),
                        &[],
                        &[String::from("test_f")],
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new()
                    )
                    .await
                    .is_err());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{ElTarget, Element, Join, Metadata, Task, Values},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
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

    fn reading(join: Join) {
        get_rt().block_on(async {
            let origin = format!("test {{\n{join};\n}};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    while let Some(task) = src.report_err_if(Task::dissect(reader))? {
                        assert_eq!(format!("{task};"), origin);
                    }
                    Ok::<(), LinkedErr<E>>(())
                }
            );
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
            reading(args.clone());
        }
    }
}
