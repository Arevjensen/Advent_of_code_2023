use std::fmt::{Debug, Display, Formatter, Result};
use Solution::*;

#[derive(Clone)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I8(x) => std::fmt::Display::fmt(&x, f),
            I16(x) => std::fmt::Display::fmt(&x, f),
            I32(x) => std::fmt::Display::fmt(&x, f),
            I64(x) => std::fmt::Display::fmt(&x, f),
            I128(x) => Debug::fmt(&x, f),
            Isize(x) => std::fmt::Display::fmt(&x, f),
            U8(x) => std::fmt::Display::fmt(&x, f),
            U16(x) => std::fmt::Display::fmt(&x, f),
            U32(x) => std::fmt::Display::fmt(&x, f),
            U64(x) => std::fmt::Display::fmt(&x, f),
            U128(x) => std::fmt::Display::fmt(&x, f),
            Usize(x) => std::fmt::Display::fmt(&x, f),
            Str(x) => std::fmt::Display::fmt(&x, f),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::I8(l0), Self::I8(r0)) => l0 == r0,
            (Self::I16(l0), Self::I16(r0)) => l0 == r0,
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::I128(l0), Self::I128(r0)) => l0 == r0,
            (Self::Isize(l0), Self::Isize(r0)) => l0 == r0,
            (Self::U8(l0), Self::U8(r0)) => l0 == r0,
            (Self::U16(l0), Self::U16(r0)) => l0 == r0,
            (Self::U32(l0), Self::U32(r0)) => l0 == r0,
            (Self::U64(l0), Self::U64(r0)) => l0 == r0,
            (Self::U128(l0), Self::U128(r0)) => l0 == r0,
            (Self::Usize(l0), Self::Usize(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::I8(arg0) => f.debug_tuple("I8").field(arg0).finish(),
            Self::I16(arg0) => f.debug_tuple("I16").field(arg0).finish(),
            Self::I32(arg0) => f.debug_tuple("I32").field(arg0).finish(),
            Self::I64(arg0) => f.debug_tuple("I64").field(arg0).finish(),
            Self::I128(arg0) => f.debug_tuple("I128").field(arg0).finish(),
            Self::Isize(arg0) => f.debug_tuple("Isize").field(arg0).finish(),
            Self::U8(arg0) => f.debug_tuple("U8").field(arg0).finish(),
            Self::U16(arg0) => f.debug_tuple("U16").field(arg0).finish(),
            Self::U32(arg0) => f.debug_tuple("U32").field(arg0).finish(),
            Self::U64(arg0) => f.debug_tuple("U64").field(arg0).finish(),
            Self::U128(arg0) => f.debug_tuple("U128").field(arg0).finish(),
            Self::Usize(arg0) => f.debug_tuple("Usize").field(arg0).finish(),
            Self::Str(arg0) => f.debug_tuple("Str").field(arg0).finish(),
        }
    }
}
