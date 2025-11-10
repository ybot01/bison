use alloc::vec::Vec;
use crate::global_error::{GlobalError, GlobalResult};

pub struct VecWrapper{
    bytes: Vec<u8>,
    index: usize
}

impl<T: Into<Vec<u8>>> From<T> for VecWrapper{
    fn from(bytes: T) -> Self {
        Self {
            bytes: bytes.into(),
            index: 0
        }
    }
}

impl VecWrapper{

    pub fn read_bool(&mut self) -> GlobalResult<bool>{
        self.read_u8().map(|x| x != 0)
    }

    pub fn read_u8(&mut self) -> GlobalResult<u8>{
        self.bytes.get(self.index).ok_or(GlobalError::RETURNED_NONE)
            .inspect(|_| self.index += 1).map(|x| *x)
    }

    pub fn read_u16(&mut self) -> GlobalResult<u16>{
        self.read_bytes().map(u16::from_be_bytes)
    }

    pub fn read_u32(&mut self) -> GlobalResult<u32>{
        self.read_bytes().map(u32::from_be_bytes)
    }

    pub fn read_u64(&mut self) -> GlobalResult<u64>{
        self.read_bytes().map(u64::from_be_bytes)
    }

    pub fn read_u128(&mut self) -> GlobalResult<u128>{
        self.read_bytes().map(u128::from_be_bytes)
    }

    pub fn read_i8(&mut self) -> GlobalResult<i8>{
        self.read_bytes().map(i8::from_be_bytes)
    }

    pub fn read_i16(&mut self) -> GlobalResult<i16>{
        self.read_bytes().map(i16::from_be_bytes)
    }

    pub fn read_i32(&mut self) -> GlobalResult<i32>{
        self.read_bytes().map(i32::from_be_bytes)
    }

    pub fn read_i64(&mut self) -> GlobalResult<i64>{
        self.read_bytes().map(i64::from_be_bytes)
    }

    pub fn read_f64(&mut self) -> GlobalResult<f64>{
        self.read_bytes().map(f64::from_be_bytes)
    }

    pub fn read_bytes<const N: usize>(&mut self) -> GlobalResult<[u8;N]>{
        Ok(<[u8;N]>::try_from(self.bytes.get(self.index..(self.index+N)).ok_or(GlobalError::RETURNED_NONE)?)
            .inspect(|_| self.index += N )?)
    }

    pub fn read_vec(&mut self, bytes: usize) -> GlobalResult<Vec<u8>>{
        Ok(self.bytes.get(self.index..(self.index+bytes)).ok_or(GlobalError::RETURNED_NONE).inspect(|_| self.index += bytes)?.to_vec())
    }

    pub fn is_finished(&self) -> bool{
        self.index >= self.bytes.len()
    }
}