use std::{cmp::Ordering, fmt::Debug};
use ormlite::database::HasArguments;
use sqlx::{Encode, Type};
use sqlx::encode::IsNull;

/// For values automatically generated by database
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AutoGen<T>
where
    T: core::fmt::Debug + PartialEq,
{
    Known(T),
    Unknown,
}

impl<T> From<AutoGen<T>> for Option<T> where
    T: core::fmt::Debug + PartialEq, {
    fn from(value: AutoGen<T>) -> Option<T> {
        match value {
            AutoGen::Known(v) => Some(v),
            AutoGen::Unknown => None
        }
    }
}

impl<T, DB: sqlx::Database> Encode<'_,DB> for AutoGen<T> where
    T: core::fmt::Debug + PartialEq + for <'a> sqlx::Encode<'a, DB>,
    Self: Copy,
    Option<T>: for <'a> Encode<'a, DB> {
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        <Option<T> as Encode<'_, DB>>::encode_by_ref(&(*self).into(), buf)
    }
}

impl<T, DB: sqlx::Database> Type<DB> for AutoGen<T> where
    T: core::fmt::Debug + PartialEq + Type<DB>, {
    fn type_info() -> DB::TypeInfo {
        <Option<T> as Type<DB>>::type_info()
    }
}

impl<T> PartialOrd for AutoGen<T>
where
    T: PartialOrd + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (AutoGen::Known(a), AutoGen::Known(b)) => a.partial_cmp(b),
            (AutoGen::Known(_), AutoGen::Unknown) => Some(Ordering::Greater),
            (AutoGen::Unknown, AutoGen::Known(_)) => Some(Ordering::Less),
            (AutoGen::Unknown, AutoGen::Unknown) => Some(Ordering::Equal),
        }
    }
}

impl<T> From<Option<T>> for AutoGen<T>
where
    T: ts_rs::TS + core::fmt::Debug + PartialEq,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Self::Known(t),
            None => Self::Unknown,
        }
    }
}

/// `AutoGen<T>` acts like an option in JS/TS
impl<T> ts_rs::TS for AutoGen<T>
where
    T: ts_rs::TS + core::fmt::Debug + PartialEq,
{
    fn name() -> String {
        <Option<T> as ts_rs::TS>::name()
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        <Option<T> as ts_rs::TS>::name_with_type_args(args)
    }

    fn inline() -> String {
        <Option<T> as ts_rs::TS>::inline()
    }

    fn inline_flattened() -> String {
        <Option<T> as ts_rs::TS>::inline_flattened()
    }

    fn dependencies() -> Vec<ts_rs::Dependency> {
        <Option<T> as ts_rs::TS>::dependencies()
    }

    fn transparent() -> bool {
        <Option<T> as ts_rs::TS>::transparent()
    }
}
