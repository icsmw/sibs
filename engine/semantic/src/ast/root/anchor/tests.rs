use crate::tests::{analyze_globals, Files};
use crate::*;
use parser::{Parser, ReadNode, TryReadOneOf};

test_semantic_files!(
    preceding_declarations_and_types,
    files = {
        "globals.sibs" => "const later: num = 2; const base: num = later; global count: num = base + 1;",
    },
    |files| {
        let scx = files.analyze("globals from \"globals.sibs\";").unwrap();
        assert_eq!(
            scx.globals.lookup("count").unwrap().binding.ty(),
            Some(&DeterminedTy::Num.into())
        );
        assert_eq!(
            scx.globals.lookup("count").unwrap().kind,
            GlobalKind::Mutable
        );
        assert_eq!(
            scx.globals.lookup("base").unwrap().kind,
            GlobalKind::Constant
        );
    }
);

test_semantic_files!(invalid_initializers_are_rejected, files = {}, |files| {
    for source in [
        "global count: num = \"wrong\";",
        "const value: str = 5;",
        "const value: num = missing;",
        "const first: num = second; const second: num = first;",
        "const first: num = first;",
        "global first: num = 1; const second: num = first;",
        "global first: num = 1; global second: num = first;",
        "const first: num = 1; global first: num = 2;",
        "const first: num = call();",
        "const first: Vec<num> = [call()];",
    ] {
        files.write("globals.sibs", source);
        assert!(
            files.analyze("globals from \"globals.sibs\";").is_err(),
            "{source}"
        );
    }
});

test_semantic_files!(
    globals_are_visible_through_imports_and_all_execution_scopes,
    files = {
        "vars.sibs" => "const base: num = 1; global count: num = base;",
        "math.sibs" => "fn read() { count; }; fn write() { count += 1; count; };",
        "component.sibs" => "component other() { task run() { count = 2; count; } };",
    },
    |files| {
        let scx = files.analyze("globals from \"vars.sibs\"; mod from \"math.sibs\"; include from \"component.sibs\"; component c() { task run() { let f = |unused: num| { count; }; count += 1; count; } };").unwrap();
        assert_eq!(scx.globals.iter().count(), 2);
    }
);

test_semantic_files!(
    immutable_globals_and_shadowing_are_rejected,
    files = {
        "vars.sibs" => "const fixed: num = 1; global count: num = 0;",
    },
    |files| {
        for body in [
            "fixed = 2;",
            "fixed += 1;",
            "let count = 2;",
            "for count in [1, 2] { count; }",
        ] {
            let source =
                format!("globals from \"vars.sibs\"; component c() {{ task run() {{ {body} }} }};");
            assert!(files.analyze(&source).is_err(), "{body}");
        }
        for source in [
            "globals from \"vars.sibs\"; mod m { fn f(count: num) { count; } };",
            "globals from \"vars.sibs\"; component c() { task run(count: num) { count; } };",
        ] {
            assert!(files.analyze(source).is_err(), "{source}");
        }
    }
);

test_semantic_files!(
    envs_are_strings_without_reading_process_environment,
    files = {
        "vars.envs" => "SIBS_TEST_UNSET_ENV_51CBDE;",
        "vars.sibs" => "const label: str = SIBS_TEST_UNSET_ENV_51CBDE;",
    },
    |files| {
        let scx = files.analyze("envs from \"vars.envs\"; globals from \"vars.sibs\"; component c() { task run() { label; } };").unwrap();
        assert_eq!(
            scx.globals
                .lookup("SIBS_TEST_UNSET_ENV_51CBDE")
                .unwrap()
                .kind,
            GlobalKind::Environment
        );
        assert_eq!(
            scx.globals
                .lookup("SIBS_TEST_UNSET_ENV_51CBDE")
                .unwrap()
                .binding
                .ty(),
            Some(&DeterminedTy::Str.into())
        );
        assert!(files.analyze("envs from \"vars.envs\"; component c() { task run() { SIBS_TEST_UNSET_ENV_51CBDE = \"x\"; } };").is_err());
    }
);

test_semantic_files!(
    repeat_import_is_idempotent_but_different_declarations_conflict,
    files = {
        "vars.sibs" => "const base: num = 1;",
        "other.sibs" => "const base: num = 2;",
    },
    |files| {
        let scx = files
            .analyze(
                "globals from \"vars.sibs\"; mod m { globals from \"./vars.sibs\"; fn f() { base; } };",
            )
            .unwrap();
        assert_eq!(scx.globals.iter().count(), 1);
        assert!(files
            .analyze("globals from \"vars.sibs\"; globals from \"other.sibs\";")
            .is_err());
        files.write("vars.envs", "base;");
        assert!(files
            .analyze("globals from \"vars.sibs\"; envs from \"vars.envs\";")
            .is_err());
        files.write("vars.envs", "ENV_A; ENV_A;");
        assert!(files.analyze("envs from \"vars.envs\";").is_err());
    }
);

test_semantic_files!(
    reference_to_a_later_file_is_undefined,
    files = {
        "a.sibs" => "const first: num = second;",
        "b.sibs" => "const second: num = first;",
    },
    |files| {
        let error = files
            .analyze("globals from \"a.sibs\"; globals from \"b.sibs\";")
            .unwrap_err();
        assert!(matches!(error.e, E::VariableIsNotDefined(name) if name == "second"));
    }
);

test_semantic_files!(
    ordinary_import_cycles_return_errors,
    files = {
        "a.sibs" => "include from \"b.sibs\";",
        "b.sibs" => "include from \"a.sibs\";",
    },
    |files| {
        let parser = Parser::new(files.write("main.sibs", "include from \"a.sibs\";"), false).unwrap();
        assert!(LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Anchor])).is_err());
    }
);

test_semantic_files!(
    working_and_special_file_kinds_cannot_be_mixed,
    files = {
        "empty.sibs" => "",
    },
    |files| {
        for source in [
            "globals from \"empty.sibs\"; mod from \"empty.sibs\";",
            "mod from \"empty.sibs\"; globals from \"empty.sibs\";",
        ] {
            let parser = Parser::new(files.write("main.sibs", source), false).unwrap();
            assert!(LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Anchor])).is_err());
        }
    }
);

test_semantic_files!(
    anchor_checks_incompatible_imports_across_separate_branches,
    files = {
        "empty.sibs" => "",
        "left.sibs" => "globals from \"empty.sibs\";",
    },
    |files| {
        for right in ["envs from \"./empty.sibs\";", "mod from \"./empty.sibs\";"] {
            files.write("right.sibs", right);
            for source in [
                "include from \"left.sibs\"; include from \"right.sibs\";",
                "include from \"right.sibs\"; include from \"left.sibs\";",
            ] {
                let parser = Parser::new(files.write("main.sibs", source), false).unwrap();
                let error = Anchor::read(&parser).unwrap_err();
                assert!(matches!(
                    error.e,
                    parser::ParserError::MissedExpectation(..)
                ));
                assert!(parser
                    .get_src_content(Some(&error.link.src))
                    .unwrap()
                    .is_some());
            }
        }
    }
);

test_semantic_files!(
    constant_cannot_initialize_itself,
    files = {
        "globals.sibs" => "const self_ref: num = self_ref;",
    },
    |files| {
        let error = files.analyze("globals from \"globals.sibs\";").unwrap_err();
        assert!(matches!(error.e, E::VariableIsNotDefined(name) if name == "self_ref"));
    }
);

test_semantic_files!(
    globals_keep_their_declared_type_after_assignment,
    files = {
        "vars.sibs" => "global count: num = 0;",
    },
    |files| {
        assert!(files.analyze("globals from \"vars.sibs\"; component c() { task run() { count = 1; count = \"wrong\"; } };").is_err());
        assert!(files
            .analyze(
                "globals from \"vars.sibs\"; component c() { task run() { let wrong = \"wrong\"; count += wrong; } };"
            )
            .is_err());
    }
);

test_semantic_files!(
    diamond_imports_share_one_global_declaration,
    files = {
        "vars.sibs" => "const base: num = 1;",
        "left.sibs" => "globals from \"vars.sibs\"; fn get() { base; };",
        "right.sibs" => "globals from \"./vars.sibs\"; fn get() { base; };",
    },
    |files| {
        let scx = files
            .analyze("mod from \"left.sibs\"; mod from \"right.sibs\";")
            .unwrap();
        assert_eq!(scx.globals.iter().count(), 1);
    }
);

test_semantic_files!(
    import_tokens_belong_to_the_importing_source,
    files = {
        "vars.sibs" => "global count: num = 0;",
        "vars.envs" => "ENV_NAME;",
    },
    |files| {
        let parser = Parser::new(
            files.write(
                "main.sibs",
                "globals from \"vars.sibs\"; envs from \"vars.envs\";",
            ),
            false,
        )
        .unwrap();
        let node = LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Anchor]))
            .unwrap()
            .unwrap();
        let tokens = node.get_semantic_tokens(SemanticTokenContext::Ignored);
        assert!(!tokens.is_empty());
        assert!(tokens
            .iter()
            .all(|token| token.position.src == node.link().src));
    }
);

#[cfg(unix)]
test_semantic_files!(
    canonical_identity_preserves_working_import_relative_paths,
    files = {},
    |files| {
        std::fs::create_dir(files.0.join("physical")).unwrap();
        std::fs::create_dir(files.0.join("logical")).unwrap();
        files.write("physical/values.sibs", "const physical: num = 1;");
        files.write("logical/values.sibs", "const logical: num = 1;");
        let module = files.write(
            "physical/library.sibs",
            "globals from \"values.sibs\"; fn get() { logical; };",
        );
        std::os::unix::fs::symlink(module, files.0.join("logical/library.sibs")).unwrap();
        let scx = files.analyze("mod from \"logical/library.sibs\";").unwrap();
        assert!(scx.globals.lookup("logical").is_some());
        assert!(!scx.globals.lookup("physical").is_some());
    }
);

test_semantic_cases!(
    global_and_const_annotations_are_checked,
    |keyword: &str, value: &str, kind: GlobalKind| {
        let scx = analyze_globals(&format!("{keyword} value: num = {value};")).unwrap();
        let symbol = scx.globals.lookup("value").unwrap();
        assert_eq!(symbol.kind, kind);
        assert_eq!(symbol.binding.ty(), Some(&Ty::Determined(DeterminedTy::Num)));
        let error = analyze_globals(&format!("{keyword} value: str = {value};")).unwrap_err();
        assert!(matches!(error.e, E::DismatchTypes(_)));
    },
    global_negative => ("global", "0 - 10000", GlobalKind::Mutable),
    global_zero => ("global", "0", GlobalKind::Mutable),
    global_positive => ("global", "9999", GlobalKind::Mutable),
    const_negative => ("const", "0 - 10000", GlobalKind::Constant),
    const_zero => ("const", "0", GlobalKind::Constant),
    const_positive => ("const", "9999", GlobalKind::Constant),
);

test_semantic_cases!(
    declaration_keyword_controls_assignment,
    |keyword: &str, operator: &str, mutable: bool| {
        let files = Files::new();
        files.write("values.sibs", &format!("{keyword} value: num = 0;"));
        let source = format!("globals from \"values.sibs\"; component c() {{ task run() {{ value {operator} 7; }} }};");
        let result = files.analyze(&source);
        if mutable {
            assert!(result.is_ok(), "{result:?}");
        } else {
            assert!(matches!(result.unwrap_err().e, E::ImmutableGlobal(name) if name == "value"));
        }
    },
    global_assign => ("global", "=", true),
    global_compound_assign => ("global", "+=", true),
    const_assign => ("const", "=", false),
    const_compound_assign => ("const", "+=", false),
);

test_semantic_cases!(
    references_require_a_preceding_declaration,
    |source: &str, valid: bool| {
        let result = analyze_globals(source);
        if valid {
            let scx = result.unwrap();
            assert_eq!(scx.globals.lookup("count").unwrap().binding.ty(), Some(&Ty::Determined(DeterminedTy::Num)));
        } else {
            assert!(matches!(result.unwrap_err().e, E::VariableIsNotDefined(name) if name == "base"));
        }
    },
    preceding => ("const base: num = 2; global count: num = base + 1;", true),
    following => ("global count: num = base + 1; const base: num = 2;", false),
);

test_semantic_cases!(
    initializers_check_nested_references,
    |keyword: &str, ty: &str, expression: &str, mutable: bool| {
        let result = analyze_globals(&format!("{keyword} base: num = 2; const result: {ty} = {expression};"));
        if mutable {
            assert!(matches!(result.unwrap_err().e, E::InvalidGlobalInitializer(_)));
        } else {
            assert!(result.is_ok(), "{result:?}");
        }
    },
    const_arithmetic => ("const", "num", "(base + 1) * 2", false),
    const_comparison => ("const", "bool", "base > 0", false),
    const_array => ("const", "Vec<num>", "[base, base + 1]", false),
    const_interpolation => ("const", "str", "'value {base}'", false),
    global_arithmetic => ("global", "num", "(base + 1) * 2", true),
    global_comparison => ("global", "bool", "base > 0", true),
    global_array => ("global", "Vec<num>", "[base, base + 1]", true),
    global_interpolation => ("global", "str", "'value {base}'", true),
);

test_semantic_cases!(
    named_arguments_keep_global_types_in_the_registry,
    |keyword: &str| {
        let mut scx = analyze_globals(&format!("{keyword} count: num = 0;")).unwrap();
        let source = "count = 7";
        let mut lexer = lexer::Lexer::new(source, 0);
        let parser = Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, source, false);
        let arg = ArgumentAssignation::read(&parser).unwrap().unwrap();
        arg.initialize(&mut scx).unwrap();
        assert!(scx.tys.get().unwrap().lookup("count").is_none());
        assert_eq!(scx.globals.lookup("count").unwrap().binding.ty(), Some(&Ty::Determined(DeterminedTy::Num)));
    },
    mutable => ("global"),
    immutable => ("const"),
);

test_semantic_cases!(
    env_names_always_produce_immutable_strings,
    |names: &[&str]| {
        let source = names.iter().map(|name| format!("{name};")).collect::<String>();
        let files = Files::new();
        files.write("envs.sibs", &source);
        let scx = files.analyze("envs from \"envs.sibs\";").unwrap();
        assert_eq!(scx.globals.iter().count(), names.len());
        for name in names {
            let symbol = scx.globals.lookup(name).unwrap();
            assert_eq!(symbol.kind, GlobalKind::Environment);
            assert_eq!(symbol.binding.ty(), Some(&Ty::Determined(DeterminedTy::Str)));
            assert!(!scx.globals.is_mutable(name));
        }
    },
    empty => (&[]),
    single => (&["ENV_A"]),
    distinct_names => (&["ENV_A", "ENV_AA", "ENV_B", "ENV_BUILD", "ENV_PATH", "ENV_Z", "ENV_ABCDEFGHIJKL"]),
);

test_semantic_cases!(
    environment_names_require_a_preceding_import,
    |env_first: bool| {
        let files = Files::new();
        files.write("globals.sibs", "global count: num = 2; const label: str = ENV_TEST_ABSENT;");
        files.write("envs.sibs", "ENV_TEST_ABSENT;");
        let g = "globals from \"globals.sibs\";";
        let e = "envs from \"envs.sibs\";";
        let source = if env_first { format!("{e}{g}") } else { format!("{g}{e}") };
        let result = files.analyze(&source);
        if env_first {
            assert!(result.is_ok(), "{result:?}");
        } else {
            assert!(matches!(result.unwrap_err().e, E::VariableIsNotDefined(name) if name == "ENV_TEST_ABSENT"));
        }
    },
    preceding => (true),
    following => (false),
);

test_semantic_cases!(
    repeated_imports_register_each_source_once,
    |keyword: &str, kind: GlobalKind, env_first: bool| {
        let files = Files::new();
        files.write("values.sibs", &format!("{keyword} label: str = ENV_REPEAT;"));
        files.write("values.envs", "ENV_REPEAT;");
        // Direct paths, aliases and mixed paths; two through five repeated imports.
        for prefixes in [
            &["", ""][..],
            &["./", "./"][..],
            &["./", "", "./"][..],
            &["", "./", "", "./"][..],
            &["./", "", "./", "", "./"][..],
        ] {
            let mut source = String::from("envs from \"values.envs\"; globals from \"values.sibs\";");
            for prefix in prefixes {
                let g = format!("globals from \"{prefix}values.sibs\";");
                let e = format!("envs from \"{prefix}values.envs\";");
                source.push_str(&if env_first { format!("{e}{g}") } else { format!("{g}{e}") });
            }
            let scx = files.analyze(&source).unwrap();
            assert_eq!(scx.globals.iter().count(), 2, "{source}");
            for name in ["label", "ENV_REPEAT"] {
                let symbol = scx.globals.lookup(name).unwrap();
                assert!(scx.tys.lookup(name).unwrap().is_none());
                assert!(scx.tys.get_by_node(&symbol.binding.node).is_none());
                assert_eq!(symbol.binding.ty(), Some(&Ty::Determined(DeterminedTy::Str)));
            }
            assert_eq!(scx.globals.lookup("ENV_REPEAT").unwrap().kind, GlobalKind::Environment);
            assert_eq!(scx.globals.lookup("label").unwrap().kind, kind);
        }
    },
    global_env_first => ("global", GlobalKind::Mutable, true),
    global_env_last => ("global", GlobalKind::Mutable, false),
    const_env_first => ("const", GlobalKind::Constant, true),
    const_env_last => ("const", GlobalKind::Constant, false),
);

test_semantic_cases!(
    nested_includes_follow_declaration_order,
    |depth: usize, globals_first: bool, cycle: bool| {
        let files = Files::new();
        for i in 0..depth {
            let value = if i + 1 < depth { format!("v{} + 1", i + 1) }
                else if cycle { "v0".into() } else { "1".into() };
            files.write(&format!("values{i}.sibs"), &format!("const v{i}: num = {value};"));
            let g = format!("globals from \"values{i}.sibs\";");
            let next = if i + 1 < depth { format!("include from \"level{}.sibs\";", i + 1) } else { String::new() };
            files.write(&format!("level{i}.sibs"), &if globals_first { format!("{g}{next}") } else { format!("{next}{g}") });
        }
        let result = files.analyze("include from \"level0.sibs\"; component c() { task run() { v0; } };");
        if cycle || (globals_first && depth > 1) {
            assert!(matches!(result.unwrap_err().e, E::VariableIsNotDefined(_)));
        } else {
            let scx = result.unwrap();
            assert_eq!(scx.globals.iter().count(), depth);
            assert_eq!(scx.globals.lookup("v0").unwrap().binding.ty(), Some(&Ty::Determined(DeterminedTy::Num)));
        }
    },
    ordered_depth_1 => (1, false, false),
    ordered_depth_2 => (2, false, false),
    ordered_depth_3 => (3, false, false),
    ordered_depth_4 => (4, false, false),
    ordered_depth_5 => (5, false, false),
    reversed_depth_1 => (1, true, false),
    reversed_depth_2 => (2, true, false),
    reversed_depth_3 => (3, true, false),
    reversed_depth_4 => (4, true, false),
    reversed_depth_5 => (5, true, false),
    cycle_depth_1 => (1, false, true),
    cycle_depth_2 => (2, false, true),
    cycle_depth_3 => (3, false, true),
    cycle_depth_4 => (4, false, true),
    cycle_depth_5 => (5, false, true),
    reversed_cycle_depth_1 => (1, true, true),
    reversed_cycle_depth_2 => (2, true, true),
    reversed_cycle_depth_3 => (3, true, true),
    reversed_cycle_depth_4 => (4, true, true),
    reversed_cycle_depth_5 => (5, true, true),
);

test_semantic_cases!(
    global_visibility_follows_imports_even_in_deferred_bodies,
    |body: &str, import_first: bool| {
        let files = Files::new();
        files.write("values.sibs", "const value: num = 2;");
        files.write("function.sibs", "fn read() { value; };");
        files.write("closure.sibs", "mod m { fn read() { let f = |unused: num| { value; }; } };");
        let import = "globals from \"values.sibs\";";
        let source = if import_first { format!("{import}{body}") } else { format!("{body}{import}") };
        let result = files.analyze(&source);
        if import_first {
            assert!(result.is_ok(), "{result:?}");
        } else {
            assert!(matches!(result.unwrap_err().e, E::VariableIsNotDefined(name) if name == "value"));
        }
    },
    task_after_import => ("component c() { task run() { value; } };", true),
    task_before_import => ("component c() { task run() { value; } };", false),
    function_after_import => ("mod m { fn read() { value; } };", true),
    function_before_import => ("mod m { fn read() { value; } };", false),
    closure_after_import => ("component c() { task run() { let f = |unused: num| { value; }; } };", true),
    closure_before_import => ("component c() { task run() { let f = |unused: num| { value; }; } };", false),
    module_after_import => ("mod from \"function.sibs\";", true),
    module_before_import => ("mod from \"function.sibs\";", false),
    include_after_import => ("include from \"closure.sibs\";", true),
    include_before_import => ("include from \"closure.sibs\";", false),
);

test_semantic_cases!(
    failed_initializers_do_not_register_their_declarations,
    |keyword: &str, self_reference: bool| {
        let (ty, initializer) = if self_reference { ("num", "invalid") } else { ("str", "7") };
        let source = format!("{keyword} invalid: {ty} = {initializer};");
        let mut lexer = lexer::Lexer::new(&source, 0);
        let parser = Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, &source, false);
        let module = GlobalsModule::read(&parser).unwrap().unwrap();
        let mut scx = SemanticCx::new(false);
        let error = module.initialize(&mut scx).unwrap_err();
        assert!(scx.globals.lookup("invalid").is_none());
        if self_reference {
            assert!(matches!(error.e, E::VariableIsNotDefined(name) if name == "invalid"));
        } else {
            assert!(matches!(error.e, E::DismatchTypes(_)));
        }
    },
    global_self_reference => ("global", true),
    global_type_mismatch => ("global", false),
    const_self_reference => ("const", true),
    const_type_mismatch => ("const", false),
);
