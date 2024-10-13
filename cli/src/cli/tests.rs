use crate::{
    elements::Element,
    error::LinkedErr,
    inf::{
        journal::{Configuration, Journal},
        Context, Execute, ExecuteContext, Scope, Value,
    },
    process_file,
    reader::{error::E, Reader, Sources},
};

#[tokio::test]
async fn processing() {
    let target = std::env::current_dir()
        .expect("current folder detected")
        .join("./src/tests/cli/handle_exit.sibs");
    process_file!(
        &Configuration::logs(false),
        &target,
        |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
            assert_eq!(elements.len(), 1);
            let Some(el) = elements.first() else {
                panic!("Component isn't found");
            };
            assert!(el
                .execute(
                    ExecuteContext::unbound(cx, sc).args(&[Value::String(String::from("success"))])
                )
                .await
                .is_ok());
            Ok::<(), LinkedErr<E>>(())
        }
    );
}
