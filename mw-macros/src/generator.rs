use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ValueType {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "u32")]
    U32,
    #[serde(rename = "u64")]
    U64,
    #[serde(rename = "i32")]
    I32,
    #[serde(rename = "i64")]
    I64,
    #[serde(rename = "usize")]
    Usize,
    #[serde(rename = "isize")]
    Isize,
    #[serde(rename = "bytes")]
    Bytes,
}

impl From<&str> for ValueType {
    fn from(s: &str) -> ValueType {
        match s {
            "null" => ValueType::Null,
            "u8" => ValueType::U32,
            "u16" => ValueType::U32,
            "u32" => ValueType::U32,
            "u64" => ValueType::U64,
            "i8" => ValueType::I32,
            "i16" => ValueType::I32,
            "i32" => ValueType::I32,
            "i64" => ValueType::I64,
            "usize" => ValueType::Usize,
            "isize" => ValueType::Isize,
            _ => ValueType::Null,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MethodType {
    #[serde(rename = "async")]
    Async,
    #[serde(rename = "sync")]
    NoAsync,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Args {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: ValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ret {
    #[serde(rename = "type")]
    pub ty: ValueType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Method {
    pub name: String,
    pub ty: MethodType,
    pub arguments: Vec<Args>,
    pub ret: Ret,
}

