use std::path::PathBuf;

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
    let left = ValueRef::bool;
    for right in ValueRef::iter() {
        match right {
            ValueRef::bool => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::bool => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::Numeric,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::bool, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn string() {
    let left = ValueRef::String;
    for right in ValueRef::iter() {
        match right {
            ValueRef::String => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::String => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::String,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::Numeric, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn path_buf() {
    let left = ValueRef::PathBuf;
    for right in ValueRef::iter() {
        match right {
            ValueRef::PathBuf => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::PathBuf => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::PathBuf,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::Numeric])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn numeric() {
    let left = ValueRef::Numeric;
    for right in ValueRef::iter() {
        match right {
            ValueRef::u8
            | ValueRef::u16
            | ValueRef::u32
            | ValueRef::u64
            | ValueRef::u128
            | ValueRef::usize
            | ValueRef::i8
            | ValueRef::i16
            | ValueRef::i32
            | ValueRef::i64
            | ValueRef::i128
            | ValueRef::isize
            | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::u8
                        | ValueRef::u16
                        | ValueRef::u32
                        | ValueRef::u64
                        | ValueRef::u128
                        | ValueRef::usize
                        | ValueRef::i8
                        | ValueRef::i16
                        | ValueRef::i32
                        | ValueRef::i64
                        | ValueRef::i128
                        | ValueRef::isize
                        | ValueRef::Numeric => {
                            assert!(left.is_compatible(&right))
                        }
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::Numeric,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::Numeric, ValueRef::PathBuf])));
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_u8() {
    let left = ValueRef::u8;
    for right in ValueRef::iter() {
        match right {
            ValueRef::u8 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::u8 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u8,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(left.is_compatible(&ValueRef::OneOf(vec![ValueRef::u8, ValueRef::PathBuf])));
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_u16() {
    let left = ValueRef::u16;
    for right in ValueRef::iter() {
        match right {
            ValueRef::u16 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::u16 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u16,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::u16, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_u32() {
    let left = ValueRef::u32;
    for right in ValueRef::iter() {
        match right {
            ValueRef::u32 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::u32 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u32,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::u32, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_u64() {
    let left = ValueRef::u64;
    for right in ValueRef::iter() {
        match right {
            ValueRef::u64 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::u64 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u64,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::u64, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_u128() {
    let left = ValueRef::u128;
    for right in ValueRef::iter() {
        match right {
            ValueRef::u128 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::u128 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::u128,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::u128, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_usize() {
    let left = ValueRef::usize;
    for right in ValueRef::iter() {
        match right {
            ValueRef::usize | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::usize | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::usize,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::usize, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_i8() {
    let left = ValueRef::i8;
    for right in ValueRef::iter() {
        match right {
            ValueRef::i8 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::i8 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i8,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(left.is_compatible(&ValueRef::OneOf(vec![ValueRef::i8, ValueRef::PathBuf])));
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_i16() {
    let left = ValueRef::i16;
    for right in ValueRef::iter() {
        match right {
            ValueRef::i16 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::i16 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i16,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::i16, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_i32() {
    let left = ValueRef::i32;
    for right in ValueRef::iter() {
        match right {
            ValueRef::i32 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::i32 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i32,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::i32, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_i64() {
    let left = ValueRef::i64;
    for right in ValueRef::iter() {
        match right {
            ValueRef::i64 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::i64 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i64,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::i64, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_i128() {
    let left = ValueRef::i128;
    for right in ValueRef::iter() {
        match right {
            ValueRef::i128 | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::i128 | ValueRef::Numeric => {
                            assert!(left.is_compatible(&right))
                        }
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::i128,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::i128, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn num_isize() {
    let left = ValueRef::isize;
    for right in ValueRef::iter() {
        match right {
            ValueRef::isize | ValueRef::Numeric => assert!(left.is_compatible(&right)),
            ValueRef::Optional(..) => {
                for inner in ValueRef::iter() {
                    let right = ValueRef::Optional(Box::new(inner.clone()));
                    match inner {
                        ValueRef::isize | ValueRef::Numeric => assert!(left.is_compatible(&right)),
                        _ => assert!(!left.is_compatible(&right)),
                    }
                }
            }
            ValueRef::Repeated(..) => {
                assert!(left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::isize,))));
                assert!(!left.is_compatible(&ValueRef::Repeated(Box::new(ValueRef::bool,))))
            }
            ValueRef::OneOf(..) => {
                assert!(
                    left.is_compatible(&ValueRef::OneOf(vec![ValueRef::isize, ValueRef::PathBuf]))
                );
                assert!(!left
                    .is_compatible(&ValueRef::OneOf(vec![ValueRef::String, ValueRef::PathBuf])))
            }
            _ => assert!(!left.is_compatible(&right)),
        };
    }
}

#[test]
fn vec() {
    for left_inner in ValueRef::iter() {
        let left = ValueRef::Vec(Box::new(left_inner.clone()));
        for right_inner in ValueRef::iter() {
            let right = ValueRef::Vec(Box::new(right_inner.clone()));
            if left.is_compatible(&right) != left_inner.is_compatible(&right_inner) {
                println!("LEFT: {left:?}");
                println!("RIGHT: {right:?}");
                println!("LEFT INNER: {left_inner:?}");
                println!("RIGHT INNER: {right_inner:?}");
            }
            assert_eq!(
                left.is_compatible(&right),
                left_inner.is_compatible(&right_inner)
            );
        }
    }
}
