#![no_std]

mod global_error;
mod vec_wrapper;

extern crate alloc;

use alloc::collections::BTreeMap;
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

#[derive(Clone, PartialEq, PartialOrd, Default, Debug)]
pub enum BISONType {
    Map(BISON),
    Array(Vec<BISONType>),
    String(String),
    Integer(i64),
    Float(f64),
    ByteArray(Vec<u8>),
    Boolean(bool),
    #[default]
    Null
}

impl Display for BISONType{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self{
            Self::Map(value) => write!(f, "{}", value.to_string()),
            Self::Array(value) => write!(f, "[{}]", value.iter().map(BISONType::to_string).collect::<Vec<String>>().join(", ")),
            Self::String(value) => write!(f, "\"{}\"", value),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Float(value) => write!(f, "{}", value),
            Self::ByteArray(value) => write!(f, "{:?}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Null => write!(f, "null")
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
    fn into(self) -> BISONType { BISONType::Integer(i64::from(self))}
}

impl Into<BISONType> for u16{
    fn into(self) -> BISONType { BISONType::Integer(i64::from(self))}
}

impl Into<BISONType> for u32{
    fn into(self) -> BISONType { BISONType::Integer(i64::from(self))}
}

impl TryInto<BISONType> for u64{

    type Error = GlobalError;

    fn try_into(self) -> Result<BISONType, Self::Error> {
        Ok(BISONType::Integer(i64::try_from(self)?))
    }
}

impl TryInto<BISONType> for u128{

    type Error = GlobalError;

    fn try_into(self) -> Result<BISONType, Self::Error> {
        Ok(BISONType::Integer(i64::try_from(self)?))
    }
}

impl TryInto<BISONType> for usize{

    type Error = GlobalError;

    fn try_into(self) -> Result<BISONType, Self::Error> {
        Ok(BISONType::Integer(i64::try_from(self)?))
    }
}

impl Into<BISONType> for i8{
    fn into(self) -> BISONType { BISONType::Integer(i64::from(self))}
}

impl Into<BISONType> for i16{
    fn into(self) -> BISONType { BISONType::Integer(i64::from(self))}
}

impl Into<BISONType> for i32{
    fn into(self) -> BISONType { BISONType::Integer(i64::from(self))}
}

impl Into<BISONType> for i64{
    fn into(self) -> BISONType { BISONType::Integer(self)}
}

impl TryInto<BISONType> for i128{

    type Error = GlobalError;

    fn try_into(self) -> Result<BISONType, Self::Error> {
        Ok(BISONType::Integer(i64::try_from(self)?))
    }
}

impl TryInto<BISONType> for isize{

    type Error = GlobalError;

    fn try_into(self) -> Result<BISONType, Self::Error> {
        Ok(BISONType::Integer(i64::try_from(self)?))
    }
}

impl Into<BISONType> for f32{
    fn into(self) -> BISONType { BISONType::Float(f64::from(self))}
}

impl Into<BISONType> for f64{
    fn into(self) -> BISONType { BISONType::Float(self)}
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
    const INTEGER: u8 = 3;
    const FLOAT: u8 = 4;
    const BYTE_ARRAY: u8 = 5;
    const STRING: u8 = 6;
    const ARRAY: u8 = 7;
    const MAP: u8 = 8;

    pub const fn new() -> Self {Self(BTreeMap::new())}

    pub fn keys(&self) -> Vec<String> {
        self.0.iter().map(|x| x.0.clone()).collect()
    }

    pub fn get(&self, key: impl Into<String>) -> Option<&BISONType>{
        self.0.get(&key.into())
    }

    pub fn get_mut(&mut self, key: impl Into<String>) -> Option<&mut BISONType> {
        self.0.get_mut(&key.into())
    }

    pub fn entries(&self) -> usize{
        self.0.len()
    }

    pub fn contains_key(&self, key: impl Into<String>) -> bool{
        self.0.contains_key(&key.into())
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<BISONType>){
        self.0.insert(key.into(), value.into());
    }

    pub fn delete(&mut self, key: impl Into<String>){
        self.0.remove(&key.into());
    }

    pub fn clear(&mut self){
        self.0.clear();
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

        fn process_value(mut wrapper: &mut VecWrapper) -> GlobalResult<BISONType>{
            Ok(match wrapper.read_u8()?{
                BISON::NULL => BISONType::Null,
                BISON::BOOLEAN => BISONType::Boolean(wrapper.read_bool()?),
                BISON::INTEGER => {
                    BISONType::Integer(
                        match wrapper.read_u8()?{
                            1 => i64::from(wrapper.read_u8()?.cast_signed()),
                            2 => i64::from(wrapper.read_i16()?),
                            4 => i64::from(wrapper.read_i32()?),
                            8 => wrapper.read_i64()?,
                            _ => return Err(GlobalError::Custom("invalid integer byte length"))
                        }
                    )
                },
                BISON::FLOAT => BISONType::Float(wrapper.read_f64()?),
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
        let mut map = Self::new();
        let mut wrapper = VecWrapper::from(value);
        while let Ok(key) = get_vec(&mut wrapper){
            map.insert(&String::from_utf8(key)?, process_value(&mut wrapper)?);
        }
        Ok(map)
    }
}

impl From<BISON> for Vec<u8>{
    fn from(value: BISON) -> Self {
        fn get_vec(to_write: &[u8]) -> Vec<u8>{
            let mut bytes = Vec::new();

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
            bytes
        }

        fn process_value(value: BISONType) -> Vec<u8>{
            let mut bytes = Vec::new();

            match value{
                BISONType::Map(value) => {
                    bytes.push(BISON::MAP);
                    bytes.extend_from_slice(&get_vec(&Vec::from(value)));
                }
                BISONType::Array(value) => {
                    bytes.push(BISON::ARRAY);
                    let mut array = Vec::new();
                    for single_value in value{
                        array.extend_from_slice(&process_value(single_value));
                    }
                    bytes.extend_from_slice(&get_vec(&array));
                }
                BISONType::String(value) => {
                    bytes.push(BISON::STRING);
                    bytes.extend_from_slice(&get_vec(value.as_bytes()));
                }
                BISONType::Integer(value) => {
                    bytes.push(BISON::INTEGER);
                    if let Ok(result) = i8::try_from(value){
                        bytes.push(1);
                        bytes.push(result.cast_unsigned());
                    }
                    else if let Ok(result) = i16::try_from(value){
                        bytes.push(2);
                        bytes.extend_from_slice(&result.to_be_bytes());
                    }
                    else if let Ok(result) = i32::try_from(value){
                        bytes.push(4);
                        bytes.extend_from_slice(&result.to_be_bytes());
                    }
                    else{
                        bytes.push(8);
                        bytes.extend_from_slice(&value.to_be_bytes());
                    }
                }
                BISONType::Float(value) => {
                    bytes.push(BISON::FLOAT);
                    bytes.extend_from_slice(&value.to_be_bytes());
                }
                BISONType::ByteArray(value) => {
                    bytes.push(BISON::BYTE_ARRAY);
                    bytes.extend_from_slice(&get_vec(&value));
                }
                BISONType::Boolean(value) => {
                    bytes.push(BISON::BOOLEAN);
                    bytes.push(if value {1} else {0});
                }
                BISONType::Null => bytes.push(BISON::NULL)
            }
            bytes
        }

        let mut bytes = Vec::new();

        for (key, value) in value.0{
            bytes.extend_from_slice(&get_vec(key.as_bytes()));
            bytes.extend_from_slice(&process_value(value));
        }
        bytes
    }
}