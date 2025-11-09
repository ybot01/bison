#![no_std]

mod global_error;
mod vec_wrapper;

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::collections::btree_map::Keys;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use crate::global_error::{GlobalError, GlobalResult};
use crate::vec_wrapper::VecWrapper;

#[derive(Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct BISON(BTreeMap<String, BISONType>);

impl Display for BISON{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string_internal(1))
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum BISONType {
    Map(BISON),
    Array(Vec<BISONType>),
    String(String),
    Number(String),
    ByteArray(Vec<u8>),
    Boolean(bool),
    Null
}

impl Display for BISONType{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self{
            BISONType::Map(value) => write!(f, "{}", value.to_string()),
            BISONType::Array(value) => write!(f, "[{}]", value.iter().map(BISONType::to_string).collect::<Vec<String>>().join(", ")),
            BISONType::String(value) => write!(f, "\"{}\"", value),
            BISONType::Number(value) => write!(f, "{}", value),
            BISONType::ByteArray(value) => write!(f, "{:?}", value),
            BISONType::Boolean(value) => write!(f, "{}", value),
            BISONType::Null => write!(f, "null")
        }
    }
}

impl Into<BISONType> for BISON{
    fn into(self) -> BISONType { BISONType::Map(self)}
}

impl Into<BISONType> for Vec<BISONType>{
    fn into(self) -> BISONType { BISONType::Array(self)}
}

impl Into<BISONType> for String{
    fn into(self) -> BISONType { BISONType::String(self)}
}

impl Into<BISONType> for &str{
    fn into(self) -> BISONType { BISONType::String(self.to_string())}
}

impl Into<BISONType> for u8{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for u16{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for u32{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for u64{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for u128{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for usize{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for i8{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for i16{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for i32{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for i64{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for i128{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for isize{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for f32{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for f64{
    fn into(self) -> BISONType { BISONType::Number(self.to_string())}
}

impl Into<BISONType> for Vec<u8>{
    fn into(self) -> BISONType { BISONType::ByteArray(self)}
}

impl Into<BISONType> for bool{
    fn into(self) -> BISONType { BISONType::Boolean(self)}
}

impl BISON{
    const NULL: u8 = 0;
    const BOOLEAN: u8 = 1;
    const NUMBER: u8 = 2;
    const BYTE_ARRAY: u8 = 3;
    const STRING: u8 = 4;
    const ARRAY: u8 = 5;
    const MAP: u8 = 6;

    pub const fn new() -> Self {Self(BTreeMap::new())}

    pub fn keys(&self) -> Keys<'_, String, BISONType> {self.0.keys()}

    pub fn get(&self, key: impl Into<String>) -> Option<BISONType>{
        self.0.get(&key.into()).map(|x| x.clone())
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<BISONType>){
        self.0.insert(key.into(), value.into());
    }

    pub fn delete(&mut self, key: impl Into<String>){
        self.0.remove(&key.into());
    }

    fn to_string_internal(&self, indent_level: usize) -> String{
        let mut to_return = "{\r\n".to_string();
        for (key, value) in self.0.iter(){
            for _ in 0..indent_level {to_return += "   "}
            to_return += "\"";
            to_return += key;
            to_return += "\" : ";
            fn value_as_string(value: &BISONType, indent_level: usize) -> String{
                match value{
                    BISONType::Map(value) => value.to_string_internal(indent_level + 1),
                    _ => value.to_string()
                }
            }
            to_return += &value_as_string(value, indent_level);
            to_return += ",\r\n";
        }
        if self.0.len() > 0{
            to_return.remove(to_return.len() - 3);
        }
        for _ in 0..(indent_level - 1) {to_return += "   "}
        to_return += "}";
        to_return
    }
}

impl TryFrom<Vec<u8>> for BISON{
    type Error = GlobalError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        fn process_value(mut wrapper: &mut VecWrapper) -> GlobalResult<BISONType>{
            fn get_vec(wrapper: &mut VecWrapper) -> GlobalResult<Vec<u8>>{
                let length_u8 = wrapper.read_u8()?;
                let length = match length_u8{
                    252 => usize::from(wrapper.read_u16()?),
                    253 => usize::try_from(wrapper.read_u32()?)?,
                    254 => usize::try_from(wrapper.read_u64()?)?,
                    u8::MAX => usize::try_from(wrapper.read_u128()?)?,
                    _ => usize::from(length_u8)
                };
                wrapper.read_vec(length)
            }
            Ok(match wrapper.read_u8()?{
                BISON::NULL => BISONType::Null,
                BISON::BOOLEAN => BISONType::Boolean(wrapper.read_bool()?),
                BISON::NUMBER => BISONType::Number(String::from_utf8(get_vec(&mut wrapper)?)?),
                BISON::BYTE_ARRAY => BISONType::ByteArray(get_vec(&mut wrapper)?),
                BISON::STRING => BISONType::String(String::from_utf8(get_vec(&mut wrapper)?)?),
                BISON::ARRAY => {
                    let mut array_bytes = VecWrapper::from(get_vec(&mut wrapper)?);
                    let mut array = Vec::new();
                    while !array_bytes.is_finished(){
                        array.push(process_value(&mut array_bytes)?);
                    }
                    BISONType::Array(array)
                }
                BISON::MAP => BISONType::Map(BISON::try_from(get_vec(&mut wrapper)?)?),
                _ => return Err(GlobalError::Custom("invalid value type"))
            })
        }
        let mut map = BTreeMap::new();
        let mut wrapper = VecWrapper::from(value);
        let mut key;
        while let Ok(key_length) = wrapper.read_u8(){
            key = String::from_utf8(wrapper.read_vec(usize::from(key_length))?)?;
            map.insert(key, process_value(&mut wrapper)?);
        }
        Ok(Self(map))
    }
}

impl TryFrom<BISON> for Vec<u8>{
    type Error = GlobalError;

    fn try_from(value: BISON) -> Result<Self, Self::Error> {
        fn process_value(value: BISONType) -> GlobalResult<Vec<u8>>{
            let mut bytes = Vec::new();
            fn write_vec(bytes: &mut Vec<u8>, to_write: Vec<u8>){
                if let Ok(len) = u8::try_from(to_write.len()) && (len <= 251){
                    bytes.push(len);
                }
                else if let Ok(len) = u16::try_from(to_write.len()){
                    bytes.push(252);
                    bytes.extend_from_slice(&len.to_be_bytes());
                }
                else if let Ok(len) = u32::try_from(to_write.len()){
                    bytes.push(253);
                    bytes.extend_from_slice(&len.to_be_bytes());
                }
                else if let Ok(len) = u64::try_from(to_write.len()){
                    bytes.push(254);
                    bytes.extend_from_slice(&len.to_be_bytes());
                }
                else if let Ok(len) = u128::try_from(to_write.len()){
                    bytes.push(u8::MAX);
                    bytes.extend_from_slice(&len.to_be_bytes());
                }
                bytes.extend_from_slice(&to_write);
            }
            match value{
                BISONType::Map(value) => {
                    bytes.push(BISON::MAP);
                    write_vec(&mut bytes, Vec::try_from(value)?);
                }
                BISONType::Array(value) => {
                    bytes.push(BISON::ARRAY);
                    let mut array = Vec::new();
                    for single_value in value{
                        array.extend_from_slice(&process_value(single_value)?);
                    }
                    write_vec(&mut bytes, array);
                }
                BISONType::String(value) => {
                    bytes.push(BISON::STRING);
                    write_vec(&mut bytes, value.as_bytes().to_vec());
                }
                BISONType::Number(value) => {
                    bytes.push(BISON::NUMBER);
                    write_vec(&mut bytes, value.as_bytes().to_vec());
                }
                BISONType::ByteArray(value) => {
                    bytes.push(BISON::BYTE_ARRAY);
                    write_vec(&mut bytes, value);
                }
                BISONType::Boolean(value) => {
                    bytes.push(BISON::BOOLEAN);
                    bytes.push(if value {1} else {0});
                }
                BISONType::Null => bytes.push(BISON::NULL)
            }
            Ok(bytes)
        }

        let mut bytes = Vec::new();

        for (key, value) in value.0{
            let key_bytes = key.as_bytes();
            bytes.push(u8::try_from(key_bytes.len())?);
            bytes.extend_from_slice(key_bytes);
            bytes.extend_from_slice(&process_value(value)?);
        }
        Ok(bytes)
    }
}