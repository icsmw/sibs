use crate::{
    elements::Element,
    error::LinkedErr,
    inf::{context::Context, Operator},
    reader::{error::E, read_file},
};

#[tokio::test]
async fn reading() -> Result<(), LinkedErr<E>> {
    let target = std::env::current_dir()
        .unwrap()
        .join("./src/tests/cli/handle_exit.sibs");
    let mut cx = Context::create().bound(&target)?;
    let filename = cx.scenario.filename.to_owned();
    match read_file(&mut cx, filename, true).await {
        Ok(components) => {
            assert_eq!(components.len(), 1);
            let Some(Element::Component(el, _md)) = components.first() else {
                panic!("Component isn't found");
            };
            let result = el
                .execute(None, &[], &[String::from("success")], &mut cx)
                .await;
            let _ = cx.tracker.shutdown().await;
            println!(">>>>>>>>>>>: {result:?}");
            assert!(result.is_ok());
        }
        Err(err) => {
            cx.sources.report_error(&err)?;
            let _ = cx.tracker.shutdown().await;
            return Err(err);
        }
    }
    Ok(())
}
