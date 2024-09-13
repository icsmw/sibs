pub const SEMICOLON: char = ';';
pub const OPEN_CURLY_BRACE: char = '{';
pub const CLOSE_CURLY_BRACE: char = '}';
pub const QUESTION: char = '?';
pub const COLON: char = ':';
pub const DOLLAR: char = '$';
pub const AT: char = '@';
pub const POUND_SIGN: char = '#';
pub const OPEN_BRACKET: char = '(';
pub const CLOSE_BRACKET: char = ')';
pub const OPEN_SQ_BRACKET: char = '[';
pub const CLOSE_SQ_BRACKET: char = ']';
pub const EXCLAMATION: char = '!';
pub const EQUAL: char = '=';
pub const CARET: char = '\n';
pub const SERIALIZING: char = '\\';
pub const QUOTES: char = '"';
pub const TILDA: char = '`';
pub const WS: char = ' ';
pub const COMMA: char = ',';
pub const UNDERSCORE: char = '_';
pub const DASH: char = '-';
pub const DOT: char = '.';
pub const INC: char = '+';
pub const DEC: char = '-';
pub const DIV: char = '/';
pub const MLT: char = '*';

pub fn has_reserved(str: &str) -> bool {
    let reserved = [&COMMA, &SEMICOLON, &COLON];
    for char in str.chars() {
        if reserved.contains(&&char) {
            return true;
        }
    }
    false
}
