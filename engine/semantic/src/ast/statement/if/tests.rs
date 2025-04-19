use crate::*;

test_success!(
    if_statement_000,
    Block,
    r#"{
        let a = "one";
        let b = 1;
        let c = true;
        let out = if a == "one" {
            true;
        } if b == 1 {
            false;
        } if c == true {
            true;
        } else {
            false;
        };
    }"#
);

test_success!(
    if_statement_001,
    Block,
    r#"{
        let a = "one";
        let b = 1;
        let out: bool = if a == "one" {
            true;
        } if b == 1 {
            false;
        } else {
            false;
        };
    }"#
);

test_success!(
    if_statement_002,
    Block,
    r#"{
        let a = "one";
        let out: bool = if a == "one" {
            true;
        } else {
            false;
        };
    }"#
);

test_fail!(
    if_statement_000,
    Block,
    r#"{
        let a = "one";
        let b = 1;
        let c = true;
        let out = if a == "one" {
            "true";
        } if b == 1 {
            false;
        } if c == true {
            "true";
        } else {
            false;
        };
    }"#
);

test_fail!(
    if_statement_001,
    Block,
    r#"{
        let a = "one";
        let out = if a == "one" {
            "true";
        };
    }"#
);

test_fail!(
    if_statement_002,
    Block,
    r#"{
        let a = "one";
        let b = 1;
        let out: str = if a == "one" {
            true;
        } if b == 1 {
            false;
        } else {
            false;
        };
    }"#
);
