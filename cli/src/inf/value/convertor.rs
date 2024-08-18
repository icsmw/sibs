use super::Value;
use crate::functions::{TryAnyTo, E};
use std::path::PathBuf;

impl TryAnyTo<PathBuf> for Value {
    fn try_to(&self) -> Result<PathBuf, E> {
        self.as_path_buf()
            .ok_or(E::Converting(String::from("PathBuf")))
    }
}

impl TryAnyTo<Vec<PathBuf>> for Value {
    fn try_to(&self) -> Result<Vec<PathBuf>, E> {
        self.as_path_bufs()
            .ok_or(E::Converting(String::from("Vec<PathBuf>")))
    }
}

impl TryAnyTo<String> for Value {
    fn try_to(&self) -> Result<String, E> {
        self.as_string()
            .ok_or(E::Converting(String::from("String")))
    }
}

impl TryAnyTo<usize> for Value {
    fn try_to(&self) -> Result<usize, E> {
        usize::try_from(self.as_num().ok_or(E::Converting(String::from("usize")))?)
            .map_err(|_| E::Converting(String::from("isize to usize")))
    }
}

impl TryAnyTo<u128> for Value {
    fn try_to(&self) -> Result<u128, E> {
        u128::try_from(self.as_num().ok_or(E::Converting(String::from("u128")))?)
            .map_err(|_| E::Converting(String::from("isize to u128")))
    }
}

impl TryAnyTo<u64> for Value {
    fn try_to(&self) -> Result<u64, E> {
        u64::try_from(self.as_num().ok_or(E::Converting(String::from("u64")))?)
            .map_err(|_| E::Converting(String::from("isize to u64")))
    }
}

impl TryAnyTo<u32> for Value {
    fn try_to(&self) -> Result<u32, E> {
        u32::try_from(self.as_num().ok_or(E::Converting(String::from("u32")))?)
            .map_err(|_| E::Converting(String::from("isize to u32")))
    }
}

impl TryAnyTo<u16> for Value {
    fn try_to(&self) -> Result<u16, E> {
        u16::try_from(self.as_num().ok_or(E::Converting(String::from("u16")))?)
            .map_err(|_| E::Converting(String::from("isize to u16")))
    }
}

impl TryAnyTo<u8> for Value {
    fn try_to(&self) -> Result<u8, E> {
        u8::try_from(self.as_num().ok_or(E::Converting(String::from("u8")))?)
            .map_err(|_| E::Converting(String::from("isize to u8")))
    }
}

impl TryAnyTo<isize> for Value {
    fn try_to(&self) -> Result<isize, E> {
        self.as_num().ok_or(E::Converting(String::from("isize")))
    }
}

impl TryAnyTo<i128> for Value {
    fn try_to(&self) -> Result<i128, E> {
        i128::try_from(self.as_num().ok_or(E::Converting(String::from("i128")))?)
            .map_err(|_| E::Converting(String::from("isize to i128")))
    }
}

impl TryAnyTo<i64> for Value {
    fn try_to(&self) -> Result<i64, E> {
        i64::try_from(self.as_num().ok_or(E::Converting(String::from("i64")))?)
            .map_err(|_| E::Converting(String::from("isize to i64")))
    }
}

impl TryAnyTo<i32> for Value {
    fn try_to(&self) -> Result<i32, E> {
        i32::try_from(self.as_num().ok_or(E::Converting(String::from("i32")))?)
            .map_err(|_| E::Converting(String::from("isize to i32")))
    }
}

impl TryAnyTo<i16> for Value {
    fn try_to(&self) -> Result<i16, E> {
        i16::try_from(self.as_num().ok_or(E::Converting(String::from("i16")))?)
            .map_err(|_| E::Converting(String::from("isize to i16")))
    }
}

impl TryAnyTo<i8> for Value {
    fn try_to(&self) -> Result<i8, E> {
        i8::try_from(self.as_num().ok_or(E::Converting(String::from("i8")))?)
            .map_err(|_| E::Converting(String::from("isize to i8")))
    }
}

impl TryAnyTo<bool> for Value {
    fn try_to(&self) -> Result<bool, E> {
        Ok(self
            .as_bool()
            .ok_or(E::Converting(String::from("bool")))?
            .to_owned())
    }
}
