use crate::*;

test_node_grammar!(
    envs_body_grammar,
    EnvsModule::read_as_linked,
    accepts = [
        "",
        "// comment\n",
        "ENV_A; ENV_B;",
        "// header\nENV_A;\n// tail\n",
    ],
    rejects = [
        "ENV_A",
        "ENV_A: str;",
        "ENV_A = \"x\";",
        "let ENV_A;",
        "ENV_A.foo();",
    ],
);
