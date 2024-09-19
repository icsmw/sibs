mod filter;
mod for_each;
mod map;

use crate::{
    functions::{ExecutorFnDescription, E},
    inf::{Store, ValueRef},
};

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    store.insert(
        map::name(),
        ExecutorFnDescription::new(
            map::execute,
            vec![
                ValueRef::Vec(Box::new(ValueRef::OneOf(vec![
                    ValueRef::String,
                    ValueRef::bool,
                    ValueRef::PathBuf,
                    ValueRef::u8,
                    ValueRef::u16,
                    ValueRef::u32,
                    ValueRef::u64,
                    ValueRef::u128,
                    ValueRef::usize,
                    ValueRef::i8,
                    ValueRef::i16,
                    ValueRef::i32,
                    ValueRef::i64,
                    ValueRef::i128,
                    ValueRef::isize,
                ]))),
                ValueRef::Closure,
            ],
            ValueRef::Vec(Box::new(ValueRef::Closure)),
        ),
    )?;
    store.insert(
        for_each::name(),
        ExecutorFnDescription::new(
            for_each::execute,
            vec![
                ValueRef::Vec(Box::new(ValueRef::OneOf(vec![
                    ValueRef::String,
                    ValueRef::bool,
                    ValueRef::PathBuf,
                    ValueRef::u8,
                    ValueRef::u16,
                    ValueRef::u32,
                    ValueRef::u64,
                    ValueRef::u128,
                    ValueRef::usize,
                    ValueRef::i8,
                    ValueRef::i16,
                    ValueRef::i32,
                    ValueRef::i64,
                    ValueRef::i128,
                    ValueRef::isize,
                ]))),
                ValueRef::Closure,
            ],
            ValueRef::Empty,
        ),
    )?;
    store.insert(
        filter::name(),
        ExecutorFnDescription::new(
            filter::execute,
            vec![
                ValueRef::Vec(Box::new(ValueRef::OneOf(vec![
                    ValueRef::String,
                    ValueRef::bool,
                    ValueRef::PathBuf,
                    ValueRef::u8,
                    ValueRef::u16,
                    ValueRef::u32,
                    ValueRef::u64,
                    ValueRef::u128,
                    ValueRef::usize,
                    ValueRef::i8,
                    ValueRef::i16,
                    ValueRef::i32,
                    ValueRef::i64,
                    ValueRef::i128,
                    ValueRef::isize,
                ]))),
                ValueRef::Closure,
            ],
            ValueRef::Incoming,
        ),
    )?;
    Ok(())
}
