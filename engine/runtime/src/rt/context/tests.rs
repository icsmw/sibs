use super::*;

#[tokio::test]
async fn contexts_share_globals_but_keep_locals_independent() {
    let contexts = ExecutionContexts::new(&PathBuf::from("."));
    let parent = contexts.create(Uuid::new_v4());
    let child = parent.child(Uuid::new_v4()).await.unwrap();
    let sibling = contexts.clone().create(Uuid::new_v4());
    parent
        .globals()
        .register(
            "counter".into(),
            DeterminedTy::Num.into(),
            true,
            SrcLink::default(),
            RtValue::Num(1.0),
        )
        .await
        .unwrap();
    parent.scopes().enter(&Uuid::new_v4()).await.unwrap();
    parent
        .values()
        .insert("local", RtValue::Num(10.0))
        .await
        .unwrap();
    for context in [&child, &sibling] {
        assert_eq!(
            context
                .globals()
                .lookup("counter")
                .await
                .unwrap()
                .as_deref(),
            Some(&RtValue::Num(1.0))
        );
        assert!(context.values().lookup("local").await.unwrap().is_none());
    }
    child
        .globals()
        .update("counter", |_| Ok(RtValue::Num(2.0)))
        .await
        .unwrap();
    assert_eq!(
        parent.globals().lookup("counter").await.unwrap().as_deref(),
        Some(&RtValue::Num(2.0))
    );
    child.close().await.unwrap();
    parent.close().await.unwrap();
    assert_eq!(
        sibling
            .globals()
            .lookup("counter")
            .await
            .unwrap()
            .as_deref(),
        Some(&RtValue::Num(2.0))
    );
    sibling.close().await.unwrap();
    let next = contexts.create(Uuid::new_v4());
    assert_eq!(
        next.globals().lookup("counter").await.unwrap().as_deref(),
        Some(&RtValue::Num(2.0))
    );
    contexts.destroy().await.unwrap();
}

#[tokio::test]
async fn separate_execution_contexts_have_independent_globals() {
    let first = ExecutionContexts::new(&PathBuf::from("."));
    let second = ExecutionContexts::new(&PathBuf::from("."));
    let owner = Uuid::new_v4();
    let first_context = first.create(owner);
    let second_context = second.create(owner);
    first_context
        .globals()
        .register(
            "counter".into(),
            DeterminedTy::Num.into(),
            true,
            SrcLink::default(),
            RtValue::Num(1.0),
        )
        .await
        .unwrap();
    assert!(second_context
        .globals()
        .lookup("counter")
        .await
        .unwrap()
        .is_none());
    second_context
        .globals()
        .register(
            "counter".into(),
            DeterminedTy::Num.into(),
            true,
            SrcLink::default(),
            RtValue::Num(2.0),
        )
        .await
        .unwrap();
    assert_eq!(
        first_context
            .globals()
            .lookup("counter")
            .await
            .unwrap()
            .as_deref(),
        Some(&RtValue::Num(1.0))
    );
    first.destroy().await.unwrap();
    assert_eq!(
        second_context
            .globals()
            .lookup("counter")
            .await
            .unwrap()
            .as_deref(),
        Some(&RtValue::Num(2.0))
    );
    second.destroy().await.unwrap();
}
