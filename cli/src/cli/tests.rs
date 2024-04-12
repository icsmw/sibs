use crate::{
    elements::Element,
    error::LinkedErr,
    inf::{context::Context, Operator},
    reader::{error::E, Reader, Sources},
};

#[tokio::test]
async fn reading() -> Result<(), LinkedErr<E>> {
    let target = std::env::current_dir()
        .unwrap()
        .join("./src/tests/cli/handle_exit.sibs");
    let mut cx = Context::create().unbound()?;
    let mut src = Sources::new();
    match Reader::read_file(&target, true, Some(&mut src)).await {
        Ok(components) => {
            assert_eq!(components.len(), 1);
            let Some(Element::Component(el, _md)) = components.first() else {
                panic!("Component isn't found");
            };
            let result = el
                .execute(None, &[], &[String::from("success")], &mut cx)
                .await;
            assert!(result.is_ok());
        }
        Err(err) => {
            src.report_err(&err)?;
            return Err(err);
        }
    }
    Ok(())
}
