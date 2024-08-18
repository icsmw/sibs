use tokio_util::sync::CancellationToken;

use crate::{
    elements::Element,
    error::LinkedErr,
    inf::{
        journal::{Configuration, Journal},
        AnyValue, Context, Execute, Scope,
    },
    process_file,
    reader::{error::E, Reader, Sources},
};

#[tokio::test]
async fn reading() {
    let target = std::env::current_dir()
        .expect("current folder detected")
        .join("./src/tests/cli/handle_exit.sibs");
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
                    None,
                    &[],
                    &[AnyValue::String(String::from("success"))],
                    cx,
                    sc,
                    CancellationToken::new()
                )
                .await
                .is_ok());
            Ok::<(), LinkedErr<E>>(())
        }
    );
}
