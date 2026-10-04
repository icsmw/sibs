use super::*;
use std::{
    future::{poll_fn, Future},
    sync::atomic::{AtomicUsize, Ordering},
    task::Poll,
};

macro_rules! test_global_context {
    ($name:ident, |$contexts:ident, $context:ident| $body:block) => {
        test_global_context!(
            #[tokio::test]
            $name,
            |$contexts, $context| $body
        );
    };
    (#[$attribute:meta] $name:ident, |$contexts:ident, $context:ident| $body:block) => {
        #[$attribute]
        async fn $name() {
            tokio::time::timeout(std::time::Duration::from_secs(10), async {
                let $contexts = ExecutionContexts::new(&PathBuf::from("."));
                let $context = $contexts.create(Uuid::new_v4());
                $context
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
                $body
            })
            .await
            .expect("Global access must make progress");
        }
    };
}

fn increment(value: &RtValue) -> Result<RtValue, E> {
    let RtValue::Num(value) = value else {
        return Err(E::InvalidValueType("num".into()));
    };
    Ok(RtValue::Num(value + 1.0))
}

test_global_context!(
    updates_validate_names_mutability_and_types,
    |contexts, context| {
        let globals = context.globals();
        globals
            .register(
                "constant".into(),
                DeterminedTy::Num.into(),
                false,
                SrcLink::default(),
                RtValue::Num(1.0),
            )
            .await
            .unwrap();
        for name in ["constant", "missing"] {
            let result = globals
                .update(name, |_| {
                    panic!("Invalid target must not invoke the transformation")
                })
                .await;
            assert!(matches!(
                (name, result),
                ("constant", Err(E::ImmutableGlobal(_)))
                    | ("missing", Err(E::UndefinedVariable(_)))
            ));
        }
        let first = globals.update("counter", increment).await.unwrap();
        assert_eq!(*first, RtValue::Num(2.0));
        let replacement = RtValue::Num(3.0);
        let second = globals
            .update("counter", move |_| Ok(replacement))
            .await
            .unwrap();
        assert_eq!(*second, RtValue::Num(3.0));
        assert_eq!(*first, RtValue::Num(2.0));
        assert!(matches!(
            globals
                .update("counter", |_| Ok(RtValue::Str("wrong".into())))
                .await,
            Err(E::InvalidValueType(_))
        ));
        assert_eq!(
            globals.lookup("counter").await.unwrap().as_deref(),
            Some(&RtValue::Num(3.0))
        );
        assert_eq!(
            globals.lookup("constant").await.unwrap().as_deref(),
            Some(&RtValue::Num(1.0))
        );
        assert!(globals.lookup("missing").await.unwrap().is_none());
        contexts.destroy().await.unwrap();
    }
);

test_global_context!(
    array_types_are_checked_before_commit,
    |contexts, context| {
        let globals = context.globals();
        let ty: Ty = DeterminedTy::Vec(Some(Box::new(DeterminedTy::Num))).into();
        let mixed = || RtValue::Vec(vec![RtValue::Num(1.0), RtValue::Str("wrong".into())]);
        assert!(matches!(
            globals
                .register(
                    "mixed".into(),
                    ty.clone(),
                    true,
                    SrcLink::default(),
                    mixed()
                )
                .await,
            Err(E::InvalidValueType(_))
        ));
        assert!(globals.lookup("mixed").await.unwrap().is_none());
        globals
            .register(
                "array".into(),
                ty,
                true,
                SrcLink::default(),
                RtValue::Vec(vec![RtValue::Num(1.0)]),
            )
            .await
            .unwrap();
        assert!(matches!(
            globals.update("array", move |_| Ok(mixed())).await,
            Err(E::InvalidValueType(_))
        ));
        assert_eq!(
            globals.lookup("array").await.unwrap().as_deref(),
            Some(&RtValue::Vec(vec![RtValue::Num(1.0)]))
        );
        contexts.destroy().await.unwrap();
    }
);

test_global_context!(
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    transformations_are_atomic_and_return_their_own_results,
    |contexts, context| {
        let calls = Arc::new(AtomicUsize::new(0));
        let mut workers = Vec::new();
        for _ in 0..8 {
            let context = context.clone();
            let calls = calls.clone();
            workers.push(tokio::spawn(async move {
                let mut results = Vec::new();
                for _ in 0..100 {
                    let calls = calls.clone();
                    let value = context
                        .globals()
                        .update("counter", move |value| {
                            calls.fetch_add(1, Ordering::SeqCst);
                            increment(value)
                        })
                        .await
                        .unwrap();
                    let RtValue::Num(value) = *value else {
                        panic!("Counter must remain numeric");
                    };
                    results.push(value as usize);
                }
                results
            }));
        }
        let mut results = Vec::new();
        for worker in workers {
            results.extend(worker.await.unwrap());
        }
        results.sort_unstable();
        assert_eq!(results, (2..=801).collect::<Vec<_>>());
        assert_eq!(calls.load(Ordering::SeqCst), 800);
        assert_eq!(
            context
                .globals()
                .lookup("counter")
                .await
                .unwrap()
                .as_deref(),
            Some(&RtValue::Num(801.0))
        );
        contexts.destroy().await.unwrap();
    }
);

test_global_context!(
    errors_and_panics_preserve_values_and_keep_the_handler_alive,
    |contexts, context| {
        let globals = context.globals();
        assert!(matches!(
            globals
                .update("counter", |_| Err(E::NotApplicableToTypeOperation))
                .await,
            Err(E::NotApplicableToTypeOperation)
        ));
        for kind in 0..3 {
            let result = globals
                .update("counter", move |_| match kind {
                    0 => panic!("transformation failed"),
                    1 => std::panic::panic_any(String::from("owned panic")),
                    _ => std::panic::panic_any(42usize),
                })
                .await;
            let expected = [
                "transformation failed",
                "owned panic",
                "non-string panic payload",
            ][kind];
            assert!(matches!(result, Err(E::ExecutionPanicked(message)) if message == expected));
            assert_eq!(
                globals.lookup("counter").await.unwrap().as_deref(),
                Some(&RtValue::Num(1.0))
            );
        }
        context.scopes().enter(&Uuid::new_v4()).await.unwrap();
        context
            .values()
            .insert("local", RtValue::Num(10.0))
            .await
            .unwrap();
        assert_eq!(
            context.values().lookup("local").await.unwrap().as_deref(),
            Some(&RtValue::Num(10.0))
        );
        assert_eq!(
            *globals.update("counter", increment).await.unwrap(),
            RtValue::Num(2.0)
        );
        contexts.destroy().await.unwrap();
    }
);

test_global_context!(
    cancellation_before_and_after_sending_an_update,
    |contexts, context| {
        let globals = context.globals();
        let calls = Arc::new(AtomicUsize::new(0));
        let before_calls = calls.clone();
        let unsent = globals.update("counter", move |value| {
            before_calls.fetch_add(1, Ordering::SeqCst);
            increment(value)
        });
        drop(unsent);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(
            globals.lookup("counter").await.unwrap().as_deref(),
            Some(&RtValue::Num(1.0))
        );
        let after_calls = calls.clone();
        let mut sent = Box::pin(globals.update("counter", move |value| {
            after_calls.fetch_add(1, Ordering::SeqCst);
            increment(value)
        }));
        // Poll once to send the request without yielding to the handler on this single-thread runtime.
        poll_fn(|cx| {
            assert!(sent.as_mut().poll(cx).is_pending());
            Poll::Ready(())
        })
        .await;
        drop(sent);
        assert_eq!(
            globals.lookup("counter").await.unwrap().as_deref(),
            Some(&RtValue::Num(2.0))
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            *globals.update("counter", increment).await.unwrap(),
            RtValue::Num(3.0)
        );
        contexts.destroy().await.unwrap();
    }
);

test_global_context!(
    repeat_registration_preserves_values_and_runs_are_independent,
    |contexts, context| {
        let other_contexts = ExecutionContexts::new(&PathBuf::from("."));
        let other_context = other_contexts.create(Uuid::new_v4());
        let first = context.globals();
        let second = other_context.globals();
        let link = SrcLink::default();
        second
            .register(
                "counter".into(),
                DeterminedTy::Num.into(),
                true,
                link.clone(),
                RtValue::Num(1.0),
            )
            .await
            .unwrap();
        first
            .update("counter", |_| Ok(RtValue::Num(9.0)))
            .await
            .unwrap();
        first
            .register(
                "counter".into(),
                DeterminedTy::Num.into(),
                true,
                link.clone(),
                RtValue::Num(1.0),
            )
            .await
            .unwrap();
        assert!(first.is_registered("counter", &link).await.unwrap());
        assert_eq!(
            first.lookup("counter").await.unwrap().as_deref(),
            Some(&RtValue::Num(9.0))
        );
        assert_eq!(
            second.lookup("counter").await.unwrap().as_deref(),
            Some(&RtValue::Num(1.0))
        );
        assert!(matches!(
            first
                .register(
                    "counter".into(),
                    DeterminedTy::Num.into(),
                    true,
                    SrcLink::new(&Uuid::new_v4()),
                    RtValue::Num(2.0)
                )
                .await,
            Err(E::GlobalConflict(_))
        ));
        contexts.destroy().await.unwrap();
        other_contexts.destroy().await.unwrap();
    }
);

test_global_context!(shutdown_reports_channel_errors, |contexts, context| {
    contexts.destroy().await.unwrap();
    let globals = context.globals();
    assert!(globals.lookup("counter").await.is_err());
    assert!(globals
        .is_registered("counter", &SrcLink::default())
        .await
        .is_err());
    assert!(globals
        .update("counter", |_| panic!(
            "Closed context must not execute transformations"
        ))
        .await
        .is_err());
});

test_global_context!(
    dropping_contexts_finishes_already_sent_updates,
    |contexts, context| {
        let (tx, rx) = oneshot::channel();
        contexts
            .tx
            .send(Demand::Global(
                "counter".into(),
                GlobalCommand::Update(Box::new(increment), tx),
            ))
            .unwrap();
        drop(context);
        drop(contexts);
        assert_eq!(*rx.await.unwrap().unwrap(), RtValue::Num(2.0));
    }
);
