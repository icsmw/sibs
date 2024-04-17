use crate::{
    elements::Element,
    error::LinkedErr,
    inf::{
        journal::{Configuration, Journal},
        tests::*,
        Operator,
    },
    reader::{error::E, Reader, Sources},
};

#[tokio::test]
async fn reading() -> Result<(), LinkedErr<E>> {
    let target = std::env::current_dir()
        .unwrap()
        .join("./src/tests/cli/handle_exit.sibs");
    let journal = Journal::init(Configuration::logs());
    let mut src = Sources::new(&journal);
    match Reader::read_file(&target, true, Some(&mut src), &journal).await {
        Ok(components) => {
            assert_eq!(components.len(), 1);
            let Some(Element::Component(el, _md)) = components.first() else {
                panic!("Component isn't found");
            };
            execution_from_file(&target, &src, |cx, sc| {
                Box::pin(async move {
                    let result = el
                        .execute(None, &[], &[String::from("success")], cx, sc)
                        .await;
                    journal.destroy().await;
                    assert!(result.is_ok());
                    Ok(())
                })
            })
            .await?;
        }
        Err(err) => {
            src.report_err(&err)?;
            return Err(err);
        }
    }
    Ok(())
}
