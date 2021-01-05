use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ValueType {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "u32")]
    U8,
    #[serde(rename = "u32")]
    U16,
    #[serde(rename = "u32")]
    U32,
    #[serde(rename = "u64")]
    U64,
    #[serde(rename = "i32")]
    I8,
    #[serde(rename = "i32")]
    I16,
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
    #[serde(rename = "bytes")]
    BytesVec,
}

impl ValueType {
    pub fn is_plain(&self) -> bool {
        match self {
            ValueType::Null | ValueType::Bytes | ValueType::BytesVec => false,
            _ => true,
        }
    }

    pub fn is_bytes(&self) -> bool {
        match self {
            ValueType::BytesVec | ValueType::Bytes => true,
            _ => false,
        }
    }

    pub fn to_json_type(&self) -> &str {
        match self {
            ValueType::U8 => "u8",
            ValueType::U16 => "u16",
            ValueType::U32 => "u32",
            ValueType::U64 => "u64",
            ValueType::I8 => "i8",
            ValueType::I16 => "i16",
            ValueType::I32 => "i32",
            ValueType::I64 => "i64",
            ValueType::Isize => "isize",
            ValueType::Usize => "usize",
            ValueType::Null => "null",
            ValueType::Bytes => "bytes",
            ValueType::BytesVec => "bytes_vec",
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

