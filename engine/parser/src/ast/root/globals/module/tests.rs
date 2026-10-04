use crate::*;

test_node_grammar!(
    globals_body_grammar,
    GlobalsModule::read_as_linked,
    accepts = [
        "",
        "// comment\n",
        "global count: num = 0; const label: str = \"x\";",
        "// header\nglobal count: num = 0;\n// tail\n",
    ],
    rejects = [
        "required ENV_A;",
        "optional ENV_A;",
        "global count: num;",
        "global count = 0;",
        "const count: num;",
        "const count = 0;",
        "let count = 0;",
        "global count: num = 0",
        "fn f() {}",
        "envs from \"x\";",
    ],
);
