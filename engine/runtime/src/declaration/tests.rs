use crate::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancelled_function_and_closure_restore_the_callers_scope() {
    let rt = Runtime::new(
        RtParameters::default_from_cwd().unwrap(),
        TypesTable::default(),
        Fns::default(),
        Tasks::default(),
    )
    .unwrap();
    for closure in [false, true] {
        let env = rt.create_interpreter_env("scope test", None).await.unwrap();
        env.cx.scopes().enter(&Uuid::new_v4()).await.unwrap();
        env.cx
            .values()
            .insert("value", RtValue::Num(1.0))
            .await
            .unwrap();
        env.job.cancel().cancelling().await.unwrap();
        let args = vec![UserFnArgDeclaration {
            ident: "value".into(),
            ty: Ty::Determined(DeterminedTy::Num),
            link: SrcLink::default(),
        }];
        let call = FnEnv::from_interpreter_env(
            &env,
            vec![FnArgValue::new(RtValue::Num(2.0), SrcLink::default())],
            SrcLink::default(),
        );
        let exec: UserFnExecutor =
            Box::new(|_| Box::pin(async { panic!("cancelled call must not execute its body") }));
        let result = if closure {
            ClosureFnEntity {
                uuid: Uuid::new_v4(),
                args,
                result: Ty::Determined(DeterminedTy::Void),
                body: ClosureFnBody::Executor(SrcLink::default(), exec),
            }
            .execute(call)
            .await
        } else {
            UserFnEntity {
                uuid: Uuid::new_v4(),
                name: "test".into(),
                fullname: "test".into(),
                args,
                result: Ty::Determined(DeterminedTy::Void),
                body: UserFnBody::Executor(SrcLink::default(), exec),
            }
            .execute(call, &Fns::default())
            .await
        };
        assert!(matches!(
            result,
            Err(LinkedErr {
                e: E::Cancelled,
                ..
            })
        ));
        // The parameter shadows the caller's variable only while the call scope is open.
        assert_eq!(
            env.cx.values().lookup("value").await.unwrap().as_deref(),
            Some(&RtValue::Num(1.0))
        );
        env.cx.scopes().leave().await.unwrap();
        assert!(matches!(
            env.cx.scopes().leave().await,
            Err(E::AttemptToLeaveRootContextLevel)
        ));
        env.cx.close().await.unwrap();
        env.job.cancel().cancelled::<String>(None).await.unwrap();
    }
    rt.destroy().await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn task_releases_context_even_when_leaving_scope_fails() {
    let rt = Runtime::new(
        RtParameters::default_from_cwd().unwrap(),
        TypesTable::default(),
        Fns::default(),
        Tasks::default(),
    )
    .unwrap();
    let env = rt.create_interpreter_env("scope test", None).await.unwrap();
    let job = env.job.clone();
    job.start().started::<String>(None).await.unwrap();
    let saved = Arc::new(std::sync::Mutex::new(None));
    let captured = saved.clone();
    let task = TaskEntity {
        uuid: Uuid::new_v4(),
        name: "test".into(),
        master: MasterComponent::default(),
        args: vec![],
        result: Ty::Determined(DeterminedTy::Void),
        body: TaskBody::Executor(
            SrcLink::default(),
            Box::new(move |env| {
                let captured = captured.clone();
                Box::pin(async move {
                    *captured.lock().unwrap() = Some(env.cx.clone());
                    env.job.start().started::<String>(None).await.unwrap();
                    env.job.done().success::<String>(None).await.unwrap();
                    env.cx.scopes().leave().await.unwrap();
                    // Leave a marker in the context that only close() can discard.
                    env.cx
                        .cwd()
                        .set(PathBuf::from("scope-cleanup-marker"))
                        .await
                        .unwrap();
                    Ok(RtValue::Void)
                })
            }),
        ),
    };
    assert!(matches!(
        task.execute(env, vec![], &SrcLink::default()).await,
        Err(LinkedErr {
            e: E::AttemptToLeaveRootContextLevel,
            ..
        })
    ));
    let cx = saved.lock().unwrap().take().unwrap();
    assert_eq!(
        cx.cwd().get().await.unwrap(),
        std::env::current_dir().unwrap()
    );
    cx.close().await.unwrap();
    job.done().failed::<String>(None).await.unwrap();
    rt.destroy().await.unwrap();
}
