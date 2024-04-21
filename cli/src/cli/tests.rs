use crate::{
    elements::Element,
    error::LinkedErr,
    inf::{
        journal::{Configuration, Journal},
        Context, Operator, Scope,
    },
    process_file,
    reader::{error::E, Reader, Sources},
};

#[tokio::test]
async fn reading() {
    let target = std::env::current_dir()
        .unwrap()
        .join("./src/tests/cli/handle_exit.sibs");
    process_file!(
        &Configuration::logs(),
        &target,
        |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
            assert_eq!(elements.len(), 1);
            let Some(Element::Component(el, _md)) = elements.first() else {
                panic!("Component isn't found");
            };
            assert!(el
                .execute(None, &[], &[String::from("success")], cx, sc)
                .await
                .is_ok());
            Ok::<(), LinkedErr<E>>(())
        }
    );
}
