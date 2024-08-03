use crate::{
    elements::FuncArg,
    error::LinkedErr,
    functions::{ExecutorPinnedResult, E},
    inf::{tools::get_name, AnyValue, Context, Scope},
};

pub fn name() -> String {
    get_name(module_path!())
}

pub fn execute(
    args: Vec<FuncArg>,
    args_token: usize,
    cx: Context,
    sc: Scope,
) -> ExecutorPinnedResult {
    Box::pin(async move {
        if args.len() != 1 {
            return Err(LinkedErr::new(
                E::Executing(
                    name(),
                    "Expecting only one income argument as a name of signal".to_owned(),
                ),
                Some(args_token),
            ))?;
        }
        let signal = args[0].value.as_string().ok_or(args[0].err(E::Executing(
            name(),
            "Cannot extract argument as string".to_owned(),
        )))?;
        cx.signals.emit(&signal).await?;
        sc.journal
            .debug(format!("signal \"{signal}\" has been sent"));
        Ok(AnyValue::empty())
    })
}

#[cfg(test)]
mod test {
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
            .join("./src/tests/processing/signal.sibs");
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
                        Some(el),
                        &[],
                        &[String::from("run")],
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
                    3
                );
                assert_eq!(
                    sc.get_global_var("a").await?.and_then(|v| v.as_string()),
                    Some(String::from("ok a"))
                );
                assert_eq!(
                    sc.get_global_var("b").await?.and_then(|v| v.as_string()),
                    Some(String::from("ok b"))
                );
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
