use crate::*;

vec_try_to_rs!(i8);
vec_try_to_rs!(i16);
vec_try_to_rs!(i32);
vec_try_to_rs!(i64);
vec_try_to_rs!(i128);
vec_try_to_rs!(isize);
vec_try_to_rs!(u8);
vec_try_to_rs!(u16);
vec_try_to_rs!(u32);
vec_try_to_rs!(u64);
vec_try_to_rs!(u128);
vec_try_to_rs!(usize);
vec_try_to_rs!(bool);
vec_try_to_rs!(String);
vec_try_to_rs!(PathBuf);

vec_try_to_rt_value!(i8);
vec_try_to_rt_value!(i16);
vec_try_to_rt_value!(i32);
vec_try_to_rt_value!(i64);
vec_try_to_rt_value!(i128);
vec_try_to_rt_value!(isize);
vec_try_to_rt_value!(u8);
vec_try_to_rt_value!(u16);
vec_try_to_rt_value!(u32);
vec_try_to_rt_value!(u64);
vec_try_to_rt_value!(u128);
vec_try_to_rt_value!(usize);
vec_try_to_rt_value!(bool);
vec_try_to_rt_value!(String);
vec_try_to_rt_value!(PathBuf);

#[macro_export]
macro_rules! vec_try_to_rt_value {
    ($ref:expr) => {
        paste::item! {
            impl TryToRtValue for Vec<$ref> {
                fn try_to_rtv(self) -> Result<RtValue, E> {
                    Ok(RtValue::Vec(
                        self.into_iter()
                            .map(|n| n.try_to_rtv())
                            .collect::<Result<Vec<_>, _>>()?,
                    ))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! vec_try_to_rs {
    ($ref:expr) => {
        paste::item! {
            impl TryToRs<Vec<$ref>> for RtValue {
                fn try_to_rs(self) -> Result<Vec<$ref>, E> {
                    match self {
                        RtValue::Vec(n) => Ok(n
                            .into_iter()
                            .map(|n| n.try_to_rs())
                            .collect::<Result<Vec<_>, _>>()?),
                        _ => Err(E::FailCovertToRsType(
                            self.to_string(),
                            stringify!($ref).to_owned(),
                        )),
                    }
                }
            }
        }
    };
}
