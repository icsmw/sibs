use crate::*;

impl TryToRs<u8> for RtValue {
    fn try_to_rs(self) -> Result<u8, E> {
        match self {
            RtValue::Num(n) => u8::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "u8".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "u8".to_owned())),
        }
    }
}

impl TryToRs<u16> for RtValue {
    fn try_to_rs(self) -> Result<u16, E> {
        match self {
            RtValue::Num(n) => u16::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "u16".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "u16".to_owned())),
        }
    }
}

impl TryToRs<u32> for RtValue {
    fn try_to_rs(self) -> Result<u32, E> {
        match self {
            RtValue::Num(n) => u32::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "u32".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "u32".to_owned())),
        }
    }
}

impl TryToRs<u64> for RtValue {
    fn try_to_rs(self) -> Result<u64, E> {
        match self {
            RtValue::Num(n) => u64::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "u64".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "u64".to_owned())),
        }
    }
}

impl TryToRs<u128> for RtValue {
    fn try_to_rs(self) -> Result<u128, E> {
        match self {
            RtValue::Num(n) => u128::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "u128".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "u128".to_owned())),
        }
    }
}

impl TryToRs<usize> for RtValue {
    fn try_to_rs(self) -> Result<usize, E> {
        match self {
            RtValue::Num(n) => usize::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "usize".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "usize".to_owned())),
        }
    }
}

impl TryToRs<i8> for RtValue {
    fn try_to_rs(self) -> Result<i8, E> {
        match self {
            RtValue::Num(n) => i8::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "i8".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "i8".to_owned())),
        }
    }
}

impl TryToRs<i16> for RtValue {
    fn try_to_rs(self) -> Result<i16, E> {
        match self {
            RtValue::Num(n) => i16::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "i16".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "i16".to_owned())),
        }
    }
}

impl TryToRs<i32> for RtValue {
    fn try_to_rs(self) -> Result<i32, E> {
        match self {
            RtValue::Num(n) => i32::try_from(n as i64)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "i32".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "i32".to_owned())),
        }
    }
}

impl TryToRs<i64> for RtValue {
    fn try_to_rs(self) -> Result<i64, E> {
        match self {
            RtValue::Num(n) => i64::try_from(n as i128)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "i64".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "i64".to_owned())),
        }
    }
}

impl TryToRs<i128> for RtValue {
    fn try_to_rs(self) -> Result<i128, E> {
        match self {
            RtValue::Num(n) => Ok(n as i128),
            _ => Err(E::FailCovertToRsType(self.to_string(), "i128".to_owned())),
        }
    }
}

impl TryToRs<isize> for RtValue {
    fn try_to_rs(self) -> Result<isize, E> {
        match self {
            RtValue::Num(n) => isize::try_from(n as i128)
                .map_err(|_| E::FailCovertToRsType(n.to_string(), "isize".to_owned())),
            _ => Err(E::FailCovertToRsType(self.to_string(), "isize".to_owned())),
        }
    }
}

impl TryToRs<bool> for RtValue {
    fn try_to_rs(self) -> Result<bool, E> {
        match self {
            RtValue::Bool(n) => Ok(n),
            _ => Err(E::FailCovertToRsType(self.to_string(), "bool".to_owned())),
        }
    }
}

impl TryToRs<String> for RtValue {
    fn try_to_rs(self) -> Result<String, E> {
        match self {
            RtValue::Str(n) => Ok(n),
            _ => Err(E::FailCovertToRsType(self.to_string(), "String".to_owned())),
        }
    }
}

impl TryToRs<PathBuf> for RtValue {
    fn try_to_rs(self) -> Result<PathBuf, E> {
        match self {
            RtValue::Str(n) => Ok(PathBuf::from(n)),
            RtValue::PathBuf(n) => Ok(n),
            _ => Err(E::FailCovertToRsType(
                self.to_string(),
                "PathBuf".to_owned(),
            )),
        }
    }
}

impl TryToRtValue for i8 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for i16 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for i32 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for i64 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for i128 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for isize {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for u8 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for u16 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for u32 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for u64 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for u128 {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for usize {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Num(self as f64))
    }
}

impl TryToRtValue for bool {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Bool(self))
    }
}

impl TryToRtValue for String {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Str(self))
    }
}

impl TryToRtValue for PathBuf {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::PathBuf(self))
    }
}

impl TryToRtValue for () {
    fn try_to_rtv(self) -> Result<RtValue, E> {
        Ok(RtValue::Void)
    }
}
