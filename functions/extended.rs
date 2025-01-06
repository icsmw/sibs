#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod embedded {
    mod console {
        mod print {
            use crate::*;
            pub fn name() -> String {
                get_name("functions::embedded::console::print")
            }
            pub fn fullname() -> String {
                get_fullname("functions::embedded::console::print")
            }
            pub fn args() -> Vec<Ty> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Ty::Repeated(DeterminedTy::Any)]),
                )
            }
            pub fn returning() -> DeterminedTy {
                DeterminedTy::Void
            }
            pub fn executor(
                args: Vec<FnArgValue>,
                _rt: Runtime,
            ) -> RtPinnedResult<'static, LinkedErr<E>> {
                Box::pin(async move {
                    for arg in args.iter() {
                        {
                            ::std::io::_print(format_args!("{0:?}\n", arg.value));
                        };
                    }
                    Ok(RtValue::Void)
                })
            }
        }
        use crate::*;
        pub fn register(efns: &mut EFns) -> Result<(), E> {
            efns.add(
                print::fullname(),
                EmbeddedFnEntity {
                    uuid: Uuid::new_v4(),
                    fullname: print::fullname(),
                    name: print::name(),
                    args: print::args(),
                    result: print::returning(),
                    exec: print::executor,
                },
            )?;
            Ok(())
        }
    }
    mod math {
        mod sum {
            use crate::*;
            pub fn name() -> String {
                get_name("functions::embedded::math::sum")
            }
            pub fn fullname() -> String {
                get_fullname("functions::embedded::math::sum")
            }
            pub fn args() -> Vec<Ty> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Ty::Repeated(DeterminedTy::Num)]),
                )
            }
            pub fn returning() -> DeterminedTy {
                DeterminedTy::Num
            }
            pub fn executor(
                args: Vec<FnArgValue>,
                _rt: Runtime,
            ) -> RtPinnedResult<'static, LinkedErr<E>> {
                Box::pin(async move {
                    let mut sum: f64 = 0.0;
                    for arg in args.iter() {
                        if let RtValue::Num(vl) = arg.value {
                            sum += vl;
                        } else {
                            return Err(
                                LinkedErr::by_link(
                                    E::InvalidValueType(RtValueId::Num.to_string()),
                                    (&arg.link).into(),
                                ),
                            );
                        }
                    }
                    Ok(RtValue::Num(sum))
                })
            }
        }
        use crate::*;
        pub fn register(efns: &mut EFns) -> Result<(), E> {
            efns.add(
                sum::fullname(),
                EmbeddedFnEntity {
                    uuid: Uuid::new_v4(),
                    fullname: sum::fullname(),
                    name: sum::name(),
                    args: sum::args(),
                    result: sum::returning(),
                    exec: sum::executor,
                },
            )?;
            Ok(())
        }
    }
    use crate::*;
    pub fn register(efns: &mut EFns) -> Result<(), E> {
        console::register(efns)?;
        math::register(efns)?;
        Ok(())
    }
}
mod imported {
    use std::path::PathBuf;
    use crate::*;
    pub fn register(efns: &mut EFns) -> Result<(), E> {
        fn create_dir_executor(
            args: Vec<FnArgValue>,
            _rt: Runtime,
        ) -> RtPinnedResult<'static, LinkedErr<E>> {
            Box::pin(async move {
                if args.len() != 1usize {
                    return Err(LinkedErr::unlinked(E::InvalidFnArgument));
                }
                fn create_dir(path: PathBuf) -> Result<(), E> {
                    let _ = std::fs::create_dir(path);
                    Ok(())
                }
                let mut args = args
                    .into_iter()
                    .map(Some)
                    .collect::<Vec<Option<FnArgValue>>>();
                let result = create_dir(
                        args[0usize]
                            .take()
                            .unwrap()
                            .value
                            .try_to_rs()
                            .map_err(LinkedErr::unlinked)?,
                    )
                    .map_err(LinkedErr::unlinked)?;
                result.try_to_rtv().map_err(LinkedErr::unlinked)
            })
        }
        efns.add(
            "fs::create_dir".to_string(),
            EmbeddedFnEntity {
                uuid: Uuid::new_v4(),
                fullname: "fs::create_dir".to_string(),
                name: "create_dir".to_string(),
                args: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([DeterminedTy::PathBuf.into()]),
                ),
                result: DeterminedTy::Void,
                exec: create_dir_executor,
            },
        )?;
        Ok(())
    }
}
mod utils {
    pub fn get_fullname(path: &str) -> String {
        let parts = path.split("::").collect::<Vec<&str>>();
        let count = parts.len();
        parts.into_iter().skip(count.saturating_sub(2)).collect::<Vec<&str>>().join("::")
    }
    pub fn get_name(path: &str) -> String {
        path.split("::")
            .last()
            .expect("Module should have at least one member")
            .to_owned()
    }
}
pub(crate) use boxed::boxed;
pub(crate) use diagnostics::*;
pub(crate) use importer::*;
pub(crate) use runtime::error::E;
pub(crate) use runtime::*;
pub(crate) use utils::*;
pub(crate) use uuid::Uuid;
pub fn register(efns: &mut EFns) -> Result<(), E> {
    embedded::register(efns)?;
    imported::register(efns)?;
    Ok(())
}
