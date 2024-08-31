use super::*;
use strum::IntoEnumIterator;

#[test]
fn as_num() {
    assert_eq!(Value::i8(1).as_num(), Some(1));
    assert_eq!(Value::i16(1).as_num(), Some(1));
    assert_eq!(Value::i32(1).as_num(), Some(1));
    assert_eq!(Value::i64(1).as_num(), Some(1));
    assert_eq!(Value::i128(1).as_num(), Some(1));
    assert_eq!(Value::isize(1).as_num(), Some(1));
    assert_eq!(Value::u8(1).as_num(), Some(1));
    assert_eq!(Value::u16(1).as_num(), Some(1));
    assert_eq!(Value::u32(1).as_num(), Some(1));
    assert_eq!(Value::u64(1).as_num(), Some(1));
    assert_eq!(Value::u128(1).as_num(), Some(1));
    assert_eq!(Value::usize(1).as_num(), Some(1));
    assert_eq!(Value::String(String::from("123")).as_num(), Some(123));
    assert_eq!(Value::String(String::from("abc")).as_num(), None);
    assert_eq!(Value::bool(true).as_num(), None);
    assert_eq!(Value::PathBuf(PathBuf::from("test")).as_num(), None);
    assert_eq!(Value::Vec(vec![Value::i8(1)]).as_num(), None);
}

#[test]
fn as_string() {
    assert_eq!(Value::i8(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::i16(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::i32(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::i64(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::i128(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::isize(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::u8(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::u16(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::u32(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::u64(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::u128(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::usize(1).as_string(), Some("1".to_string()));
    assert_eq!(Value::bool(true).as_string(), Some("true".to_string()));
    assert_eq!(
        Value::PathBuf(PathBuf::from("test")).as_string(),
        Some("test".to_string())
    );
    assert_eq!(
        Value::String(String::from("test")).as_string(),
        Some("test".to_string())
    );
    assert_eq!(Value::Vec(vec![Value::i8(1)]).as_string(), None);
}

#[test]
fn as_bool() {
    // Testing integer types
    assert_eq!(Value::i8(1).as_bool(), Some(true));
    assert_eq!(Value::i8(0).as_bool(), Some(false));
    assert_eq!(Value::i16(1).as_bool(), Some(true));
    assert_eq!(Value::i16(0).as_bool(), Some(false));
    assert_eq!(Value::i32(1).as_bool(), Some(true));
    assert_eq!(Value::i32(0).as_bool(), Some(false));
    assert_eq!(Value::i64(1).as_bool(), Some(true));
    assert_eq!(Value::i64(0).as_bool(), Some(false));
    assert_eq!(Value::i128(1).as_bool(), Some(true));
    assert_eq!(Value::i128(0).as_bool(), Some(false));
    assert_eq!(Value::isize(1).as_bool(), Some(true));
    assert_eq!(Value::isize(0).as_bool(), Some(false));
    assert_eq!(Value::u8(1).as_bool(), Some(true));
    assert_eq!(Value::u8(0).as_bool(), Some(false));
    assert_eq!(Value::u16(1).as_bool(), Some(true));
    assert_eq!(Value::u16(0).as_bool(), Some(false));
    assert_eq!(Value::u32(1).as_bool(), Some(true));
    assert_eq!(Value::u32(0).as_bool(), Some(false));
    assert_eq!(Value::u64(1).as_bool(), Some(true));
    assert_eq!(Value::u64(0).as_bool(), Some(false));
    assert_eq!(Value::u128(1).as_bool(), Some(true));
    assert_eq!(Value::u128(0).as_bool(), Some(false));
    assert_eq!(Value::usize(1).as_bool(), Some(true));
    assert_eq!(Value::usize(0).as_bool(), Some(false));

    // Testing bool type
    assert_eq!(Value::bool(true).as_bool(), Some(true));
    assert_eq!(Value::bool(false).as_bool(), Some(false));

    // Testing String type
    assert_eq!(Value::String(String::from("true")).as_bool(), Some(true));
    assert_eq!(Value::String(String::from("false")).as_bool(), Some(false));
    assert_eq!(Value::String(String::from("TRUE")).as_bool(), Some(true));
    assert_eq!(Value::String(String::from("FALSE")).as_bool(), Some(false));
    assert_eq!(Value::String(String::from("TrUe")).as_bool(), Some(true));
    assert_eq!(Value::String(String::from("FaLsE")).as_bool(), Some(false));
    assert_eq!(Value::String(String::from("yes")).as_bool(), Some(false));
    assert_eq!(Value::String(String::from("no")).as_bool(), Some(false));

    // Testing other types should return None
    assert_eq!(Value::PathBuf(PathBuf::from("test")).as_bool(), None);
    assert_eq!(Value::Vec(vec![Value::i8(1)]).as_bool(), None);
}

#[test]
fn as_path_buf() {
    assert_eq!(
        Value::PathBuf(PathBuf::from("test")).as_path_buf(),
        Some(PathBuf::from("test"))
    );
    assert_eq!(
        Value::String(String::from("test")).as_path_buf(),
        Some(PathBuf::from("test"))
    );
    assert_eq!(Value::i8(1).as_path_buf(), None);
}

#[test]
fn as_strings() {
    assert_eq!(
        Value::Vec(vec![
            Value::String(String::from("test1")),
            Value::String(String::from("test2"))
        ])
        .as_strings(),
        Some(vec![String::from("test1"), String::from("test2")])
    );
    assert_eq!(
        Value::Vec(vec![Value::String(String::from("test1")), Value::i8(1)]).as_strings(),
        Some(vec![String::from("test1"), String::from("1")])
    );
    assert_eq!(Value::i8(1).as_strings(), None);
}

#[test]
fn as_path_bufs() {
    assert_eq!(
        Value::Vec(vec![
            Value::PathBuf(PathBuf::from("test1")),
            Value::PathBuf(PathBuf::from("test2"))
        ])
        .as_path_bufs(),
        Some(vec![PathBuf::from("test1"), PathBuf::from("test2")])
    );
    assert_eq!(
        Value::Vec(vec![
            Value::PathBuf(PathBuf::from("test1")),
            Value::String(String::from("test2"))
        ])
        .as_path_bufs(),
        Some(vec![PathBuf::from("test1"), PathBuf::from("test2")])
    );
    assert_eq!(
        Value::Vec(vec![Value::PathBuf(PathBuf::from("test1")), Value::i8(1)]).as_path_bufs(),
        None
    );
    assert_eq!(Value::i8(1).as_path_bufs(), None);
}

#[test]
fn bool() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(ValueRef::bool.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(
                    ValueRef::bool.is_compatible(&ValueRef::Optional(Box::new(ValueRef::bool,)))
                );
                assert!(
                    !ValueRef::bool.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                );
            }
            ValueRef::Repeated(..) => {
                assert!(
                    ValueRef::bool.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,)))
                );
                assert!(!ValueRef::bool
                    .is_compatible(&ValueRef::Repeated(Box::new(ValueRef::Numeric,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::bool
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::bool, ValueRef::PathBuf])));
                assert!(!ValueRef::bool
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::bool.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::bool.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::bool.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::bool.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::bool.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::bool.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::bool.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::bool.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::bool.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::bool.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::bool.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::bool.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::bool.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(!ValueRef::bool.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::bool.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::bool.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::bool.is_compatible(&ValueRef::Vec(Box::new(ValueRef::bool))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::bool
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn string() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::String.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::String
                    .is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,))));
                assert!(!ValueRef::String
                    .is_compatible(&ValueRef::Optional(Box::new(ValueRef::Numeric,))))
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::String
                    .is_compatible(&ValueRef::Repeated(Box::new(ValueRef::String,))));
                assert!(
                    !ValueRef::String.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,)))
                )
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::String
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])));
                assert!(!ValueRef::String
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::Numeric, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::String.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::String.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::String.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::String.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::String.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::String.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::String.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::String.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::String.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::String.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::String.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::String.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::String.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(!ValueRef::String.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::String.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(ValueRef::String.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::String.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::String
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn path_buf() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::PathBuf
                    .is_compatible(&ValueRef::Optional(Box::new(ValueRef::PathBuf,))));
                assert!(!ValueRef::PathBuf
                    .is_compatible(&ValueRef::Optional(Box::new(ValueRef::Numeric,))))
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::PathBuf
                    .is_compatible(&ValueRef::Repeated(Box::new(ValueRef::PathBuf,))));
                assert!(!ValueRef::PathBuf
                    .is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::PathBuf
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])));
                assert!(!ValueRef::PathBuf
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::Numeric, ValueRef::String])))
            }
            ValueRef::Empty => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(ValueRef::PathBuf.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::PathBuf.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(
                    !ValueRef::PathBuf.is_compatible(&ValueRef::Vec(Box::new(ValueRef::PathBuf)))
                )
            }
            ValueRef::Task(..) => assert!(!ValueRef::PathBuf
                .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty)))),
        };
    }
}

#[test]
fn numeric() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::Numeric.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::Numeric
                    .is_compatible(&ValueRef::Optional(Box::new(ValueRef::Numeric,))));
                assert!(!ValueRef::Numeric
                    .is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,))))
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::Numeric
                    .is_compatible(&ValueRef::Repeated(Box::new(ValueRef::Numeric,))));
                assert!(!ValueRef::Numeric
                    .is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::Numeric
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::Numeric, ValueRef::PathBuf])));
                assert!(!ValueRef::Numeric
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::Numeric.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(ValueRef::Numeric.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(ValueRef::Numeric.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(ValueRef::Numeric.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::Numeric.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::Numeric.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::Numeric.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::Numeric.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => assert!(!ValueRef::Numeric
                .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty)))),
        };
    }
}

#[test]
fn num_u8() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::u8.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::u8.is_compatible(&ValueRef::Optional(Box::new(ValueRef::u8,))));
                assert!(
                    !ValueRef::u8.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::u8.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u8,))));
                assert!(!ValueRef::u8.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::u8
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::u8, ValueRef::PathBuf])));
                assert!(!ValueRef::u8
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::u8.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(ValueRef::u8.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::u8.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::u8.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::u8.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::u8.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::u8.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::u8.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::u8.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::u8.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::u8.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::u8.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::u8.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::u8.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::u8.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::u8.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::u8.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(
                    !ValueRef::u8.is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty)))
                )
            }
        };
    }
}

#[test]
fn num_u16() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::u16.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::u16.is_compatible(&ValueRef::Optional(Box::new(ValueRef::u16,))));
                assert!(
                    !ValueRef::u16.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::u16.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u16,))));
                assert!(!ValueRef::u16.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::u16
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::u16, ValueRef::PathBuf])));
                assert!(!ValueRef::u16
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::u16.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::u16.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(ValueRef::u16.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::u16.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::u16.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::u16.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::u16.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::u16.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::u16.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::u16.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::u16.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::u16.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::u16.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::u16.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::u16.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::u16.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::u16.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::u16
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_u32() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::u32.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::u32.is_compatible(&ValueRef::Optional(Box::new(ValueRef::u32,))));
                assert!(
                    !ValueRef::u32.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::u32.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u32,))));
                assert!(!ValueRef::u32.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::u32
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::u32, ValueRef::PathBuf])));
                assert!(!ValueRef::u32
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::u32.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::u32.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::u32.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(ValueRef::u32.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::u32.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::u32.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::u32.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::u32.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::u32.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::u32.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::u32.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::u32.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::u32.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::u32.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::u32.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::u32.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::u32.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::u32
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_u64() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::u64.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::u64.is_compatible(&ValueRef::Optional(Box::new(ValueRef::u64,))));
                assert!(
                    !ValueRef::u64.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::u64.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u64,))));
                assert!(!ValueRef::u64.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::u64
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::u64, ValueRef::PathBuf])));
                assert!(!ValueRef::u64
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::u64.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::u64.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::u64.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::u64.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(ValueRef::u64.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::u64.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::u64.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::u64.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::u64.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::u64.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::u64.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::u64.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::u64.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::u64.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::u64.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::u64.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::u64.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::u64
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_u128() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::u128.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(
                    ValueRef::u128.is_compatible(&ValueRef::Optional(Box::new(ValueRef::u128,)))
                );
                assert!(
                    !ValueRef::u128.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(
                    ValueRef::u128.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u128,)))
                );
                assert!(
                    !ValueRef::u128.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,)))
                )
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::u128
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::u128, ValueRef::PathBuf])));
                assert!(!ValueRef::u128
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::u128.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::u128.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::u128.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::u128.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::u128.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(ValueRef::u128.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::u128.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::u128.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::u128.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::u128.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::u128.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::u128.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::u128.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::u128.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::u128.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::u128.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::u128.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::u128
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_usize() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::usize.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(
                    ValueRef::usize.is_compatible(&ValueRef::Optional(Box::new(ValueRef::usize,)))
                );
                assert!(!ValueRef::usize
                    .is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,))))
            }
            ValueRef::Repeated(..) => {
                assert!(
                    ValueRef::usize.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::usize,)))
                );
                assert!(
                    !ValueRef::usize.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,)))
                )
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::usize
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::usize, ValueRef::PathBuf])));
                assert!(!ValueRef::usize
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::usize.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::usize.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::usize.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::usize.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::usize.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::usize.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(ValueRef::usize.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::usize.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::usize.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::usize.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::usize.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::usize.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::usize.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::usize.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::usize.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::usize.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::usize.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::usize
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_i8() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::i8.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::i8.is_compatible(&ValueRef::Optional(Box::new(ValueRef::i8,))));
                assert!(
                    !ValueRef::i8.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::i8.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i8,))));
                assert!(!ValueRef::i8.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::i8
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::i8, ValueRef::PathBuf])));
                assert!(!ValueRef::i8
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::i8.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::i8.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::i8.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::i8.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::i8.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::i8.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::i8.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(ValueRef::i8.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::i8.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::i8.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::i8.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::i8.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::i8.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::i8.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::i8.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::i8.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::i8.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(
                    !ValueRef::i8.is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty)))
                )
            }
        };
    }
}

#[test]
fn num_i16() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::i16.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::i16.is_compatible(&ValueRef::Optional(Box::new(ValueRef::i16,))));
                assert!(
                    !ValueRef::i16.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::i16.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i16,))));
                assert!(!ValueRef::i16.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::i16
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::i16, ValueRef::PathBuf])));
                assert!(!ValueRef::i16
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::i16.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::i16.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::i16.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::i16.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::i16.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::i16.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::i16.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::i16.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(ValueRef::i16.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::i16.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::i16.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::i16.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::i16.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::i16.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::i16.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::i16.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::i16.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::i16
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_i32() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::i32.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::i32.is_compatible(&ValueRef::Optional(Box::new(ValueRef::i32,))));
                assert!(
                    !ValueRef::i32.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::i32.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i32,))));
                assert!(!ValueRef::i32.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::i32
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::i32, ValueRef::PathBuf])));
                assert!(!ValueRef::i32
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::i32.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::i32.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::i32.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::i32.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::i32.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::i32.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::i32.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::i32.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::i32.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(ValueRef::i32.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::i32.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::i32.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::i32.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::i32.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::i32.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::i32.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::i32.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::i32
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_i64() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::i64.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(ValueRef::i64.is_compatible(&ValueRef::Optional(Box::new(ValueRef::i64,))));
                assert!(
                    !ValueRef::i64.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(ValueRef::i64.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i64,))));
                assert!(!ValueRef::i64.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::i64
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::i64, ValueRef::PathBuf])));
                assert!(!ValueRef::i64
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::i64.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::i64.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::i64.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::i64.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::i64.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::i64.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::i64.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::i64.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::i64.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::i64.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(ValueRef::i64.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::i64.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::i64.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::i64.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::i64.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::i64.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::i64.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::i64
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_i128() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::i128.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(
                    ValueRef::i128.is_compatible(&ValueRef::Optional(Box::new(ValueRef::i128,)))
                );
                assert!(
                    !ValueRef::i128.is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,)))
                )
            }
            ValueRef::Repeated(..) => {
                assert!(
                    ValueRef::i128.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i128,)))
                );
                assert!(
                    !ValueRef::i128.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,)))
                )
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::i128
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::i128, ValueRef::PathBuf])));
                assert!(!ValueRef::i128
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::i128.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::i128.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::i128.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::i128.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::i128.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::i128.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::i128.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::i128.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::i128.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::i128.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::i128.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(ValueRef::i128.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(!ValueRef::i128.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::i128.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::i128.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::i128.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::i128.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::i128
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn num_isize() {
    for value_ref in ValueRef::iter() {
        match value_ref {
            ValueRef::bool => assert!(!ValueRef::isize.is_compatible(&ValueRef::bool)),
            ValueRef::Optional(..) => {
                assert!(
                    ValueRef::isize.is_compatible(&ValueRef::Optional(Box::new(ValueRef::isize,)))
                );
                assert!(!ValueRef::isize
                    .is_compatible(&ValueRef::Optional(Box::new(ValueRef::String,))))
            }
            ValueRef::Repeated(..) => {
                assert!(
                    ValueRef::isize.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::isize,)))
                );
                assert!(
                    !ValueRef::isize.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,)))
                )
            }
            ValueRef::OneOf(..) => {
                assert!(ValueRef::isize
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::isize, ValueRef::PathBuf])));
                assert!(!ValueRef::isize
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            ValueRef::Empty => assert!(!ValueRef::isize.is_compatible(&ValueRef::Empty)),
            ValueRef::u8 => assert!(!ValueRef::isize.is_compatible(&ValueRef::u8)),
            ValueRef::u16 => assert!(!ValueRef::isize.is_compatible(&ValueRef::u16)),
            ValueRef::u32 => assert!(!ValueRef::isize.is_compatible(&ValueRef::u32)),
            ValueRef::u64 => assert!(!ValueRef::isize.is_compatible(&ValueRef::u64)),
            ValueRef::u128 => assert!(!ValueRef::isize.is_compatible(&ValueRef::u128)),
            ValueRef::usize => assert!(!ValueRef::isize.is_compatible(&ValueRef::usize)),
            ValueRef::i8 => assert!(!ValueRef::isize.is_compatible(&ValueRef::i8)),
            ValueRef::i16 => assert!(!ValueRef::isize.is_compatible(&ValueRef::i16)),
            ValueRef::i32 => assert!(!ValueRef::isize.is_compatible(&ValueRef::i32)),
            ValueRef::i64 => assert!(!ValueRef::isize.is_compatible(&ValueRef::i64)),
            ValueRef::i128 => assert!(!ValueRef::isize.is_compatible(&ValueRef::i128)),
            ValueRef::isize => assert!(ValueRef::isize.is_compatible(&ValueRef::isize)),
            ValueRef::Numeric => assert!(ValueRef::isize.is_compatible(&ValueRef::Numeric)),
            ValueRef::PathBuf => assert!(!ValueRef::isize.is_compatible(&ValueRef::PathBuf)),
            ValueRef::String => assert!(!ValueRef::isize.is_compatible(&ValueRef::String)),
            ValueRef::Vec(..) => {
                assert!(!ValueRef::isize.is_compatible(&ValueRef::Vec(Box::new(ValueRef::String))))
            }
            ValueRef::Task(..) => {
                assert!(!ValueRef::isize
                    .is_compatible(&ValueRef::Task(vec![], Box::new(ValueRef::Empty))))
            }
        };
    }
}

#[test]
fn vec() {
    for left_inner in ValueRef::iter() {
        let left = ValueRef::Vec(Box::new(left_inner.clone()));
        for right_inner in ValueRef::iter() {
            let right = ValueRef::Vec(Box::new(right_inner.clone()));
            assert_eq!(
                left.is_compatible(&right),
                left_inner.is_compatible(&right_inner)
            );
        }
    }
}
