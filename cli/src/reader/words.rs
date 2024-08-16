pub const IF: &str = "if";
pub const OR: &str = "||";
pub const AND: &str = "&&";
pub const ELSE: &str = "else";
pub const FIRST: &str = "first";
pub const BREAK: &str = "break";
pub const JOIN: &str = "join";
pub const EACH: &str = "each";
pub const CMP_TRUE: &str = "==";
pub const CMP_FALSE: &str = "!=";
pub const CMP_RBIG: &str = "<";
pub const CMP_LBIG: &str = ">";
pub const CMP_RBIG_INC: &str = "<=";
pub const CMP_LBIG_INC: &str = ">=";
pub const DO_ON: &str = "=>";
pub const REF_TO: &str = "->";
pub const META: &str = "///";
pub const COMMENT: &str = "//";
pub const COMP: &str = "#(";
pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const GLOBAL_VAR: &str = "global";

pub fn is_reserved<S: AsRef<str>>(s: S) -> bool {
    [
        IF,
        OR,
        AND,
        ELSE,
        FIRST,
        BREAK,
        JOIN,
        EACH,
        CMP_TRUE,
        CMP_FALSE,
        CMP_RBIG,
        CMP_LBIG,
        CMP_RBIG_INC,
        CMP_LBIG_INC,
        DO_ON,
        REF_TO,
        META,
        COMMENT,
        COMP,
        TRUE,
        FALSE,
        GLOBAL_VAR,
    ]
    .contains(&s.as_ref())
}
