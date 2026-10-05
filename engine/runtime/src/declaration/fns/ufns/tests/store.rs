use crate::*;

fn function(uuid: Uuid) -> UserFnEntity {
    UserFnEntity {
        uuid,
        name: "helper".into(),
        args: vec![],
        result: DeterminedTy::Num.into(),
        body: UserFnBody::Declaration,
    }
}

#[test]
fn aliases_resolve_to_one_function_and_one_call_target() {
    let uuid = Uuid::new_v4();
    let mut fns = Fns::default();
    fns.ufns.enter("left");
    fns.ufns.enter("code");
    fns.ufns.add("helper", function(uuid)).unwrap();
    fns.ufns.leave();
    fns.ufns.leave();
    fns.ufns.enter("right");
    fns.ufns.enter("code");
    fns.ufns.add("helper", function(uuid)).unwrap();

    assert_eq!(fns.ufns.funcs.len(), 1);
    assert!(std::ptr::eq(
        fns.ufns.find("left::code::helper").unwrap(),
        fns.ufns.find("right::code::helper").unwrap(),
    ));
    for path in ["left::code::helper", "right::code::helper"] {
        assert_eq!(*fns.find(path).unwrap().uuid(), uuid);
    }

    // Resolving the same call through another module path preserves its target.
    let caller = Uuid::new_v4();
    for path in ["left::code::helper", "right::code::helper", "helper"] {
        assert_eq!(*fns.lookup(path, &caller).unwrap().uuid(), uuid);
        assert_eq!(fns.ufns.links.get(&caller), Some(&uuid));
        assert_eq!(*fns.lookup_by_caller(&caller).unwrap().uuid(), uuid);
    }
}

#[test]
fn registering_the_same_function_preserves_its_body_and_type() {
    let uuid = Uuid::new_v4();
    let mut fns = UFns::default();
    let mut original = function(uuid);
    original.body = UserFnBody::Executor(
        SrcLink::default(),
        Box::new(|_| Box::pin(async { Ok(RtValue::Num(7.0)) })),
    );
    fns.enter("left");
    fns.add("helper", original).unwrap();
    fns.set_result_ty("helper", DeterminedTy::Str.into())
        .unwrap();
    fns.add("helper", function(uuid)).unwrap();
    fns.leave();
    fns.enter("right");
    fns.add("helper", function(uuid)).unwrap();

    assert_eq!(fns.funcs.len(), 1);
    for path in ["left::helper", "right::helper"] {
        let entity = fns.find(path).unwrap();
        assert!(matches!(entity.body, UserFnBody::Executor(..)));
        assert_eq!(entity.result, DeterminedTy::Str.into());
    }
    fns.set_result_ty("helper", DeterminedTy::Bool.into())
        .unwrap();
    assert_eq!(
        fns.find("left::helper").unwrap().result,
        DeterminedTy::Bool.into()
    );
}

#[test]
fn conflicting_aliases_do_not_replace_existing_functions() {
    let first = Uuid::new_v4();
    let second = Uuid::new_v4();
    let mut fns = UFns::default();
    fns.add("helper", function(first)).unwrap();
    assert!(matches!(
        fns.add("helper", function(second)),
        Err(E::FuncAlreadyRegistered(name)) if name == "helper"
    ));
    assert_eq!(fns.funcs.len(), 1);
    fns.enter("other");
    fns.add("helper", function(second)).unwrap();
    fns.leave();
    assert!(matches!(
        fns.add("helper", function(second)),
        Err(E::FuncAlreadyRegistered(name)) if name == "helper"
    ));
    assert_eq!(fns.find("helper").unwrap().uuid, first);
    assert_eq!(fns.find("other::helper").unwrap().uuid, second);
}

#[test]
fn invalid_registrations_do_not_change_the_registry() {
    let uuid = Uuid::new_v4();
    let mut fns = UFns::default();
    assert!(matches!(
        fns.add("task", function(uuid)),
        Err(E::FnUsesKeyword(..))
    ));
    assert!(fns.funcs.is_empty());
    assert!(fns.find("task").is_none());
    fns.add("helper", function(uuid)).unwrap();
    assert!(matches!(
        fns.add("task", function(uuid)),
        Err(E::FnUsesKeyword(..))
    ));
    assert!(fns.find("task").is_none());
    assert_eq!(fns.funcs.len(), 1);
    assert_eq!(fns.find("helper").unwrap().uuid, uuid);
}

#[test]
fn aliases_do_not_make_signature_lookup_ambiguous() {
    let uuid = Uuid::new_v4();
    let mut fns = Fns::default();
    fns.ufns.enter("left");
    fns.ufns.add("helper", function(uuid)).unwrap();
    fns.ufns.leave();
    fns.ufns.enter("right");
    fns.ufns.add("helper", function(uuid)).unwrap();
    fns.ufns.leave();
    let caller = Uuid::new_v4();
    assert_eq!(
        *fns.lookup_by_inps("helper", &[], &caller).unwrap().uuid(),
        uuid
    );
    assert_eq!(*fns.lookup_by_caller(&caller).unwrap().uuid(), uuid);

    // Another definition with the same name and signature is still ambiguous.
    fns.ufns.enter("other");
    fns.ufns.add("helper", function(Uuid::new_v4())).unwrap();
    let ambiguous = Uuid::new_v4();
    assert!(fns.lookup_by_inps("helper", &[], &ambiguous).is_none());
    assert!(fns.lookup_by_caller(&ambiguous).is_none());
}

#[test]
fn completion_lists_paths_without_confusing_module_prefixes() {
    let uuid = Uuid::new_v4();
    let mut fns = UFns::default();
    fns.enter("code");
    fns.add("helper", function(uuid)).unwrap();
    fns.enter("nested");
    fns.add("helper", function(uuid)).unwrap();
    fns.leave();
    fns.leave();
    fns.enter("code_extra");
    fns.add("helper", function(uuid)).unwrap();

    let mut paths = fns.collect_by_path(&["code"]);
    paths.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(
        paths
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        ["helper", "nested::helper"]
    );
    assert!(paths.iter().all(|(_, entity)| entity.uuid == uuid));
    assert_eq!(fns.collect_by_path(&[]).len(), 3);
    assert!(fns.collect_by_path(&["cod"]).is_empty());
}

#[test]
fn exact_and_relative_lookup_keep_their_existing_precedence() {
    let root = Uuid::new_v4();
    let nested = Uuid::new_v4();
    let mut fns = UFns::default();
    fns.add("helper", function(root)).unwrap();
    fns.enter("code");
    fns.add("helper", function(nested)).unwrap();
    let caller = Uuid::new_v4();
    assert_eq!(fns.lookup("helper", &caller).unwrap().uuid, root);
    assert_eq!(fns.lookup("code::helper", &caller).unwrap().uuid, nested);
}
