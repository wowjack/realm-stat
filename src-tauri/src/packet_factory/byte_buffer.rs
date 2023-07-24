#![allow(dead_code)]
use byteorder::{BigEndian, ByteOrder};



#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ByteBuffer {
    pub bytes: Vec<u8>,
    pub index: usize,
}
impl ByteBuffer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, index: 0 }
    }

    pub fn reset(mut self) -> Self {
        self.index = 0;
        return self
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<&[u8], ()> {
        self.index += n;
        if self.index > self.bytes.len() {
            self.index -= n;
            return Err(())
        }
        return Ok(&self.bytes[self.index-n..self.index])
    }
    pub fn read_n_bytes_static(&self, n: usize) -> Result<&[u8], ()> {
        if self.index + n > self.bytes.len() {
            return Err(())
        }
        return Ok(&self.bytes[self.index..self.index+n])
    }

    pub fn read_u8(&mut self) -> Result<u8, ()> {
        Ok(self.read_n_bytes(1)?[0])
    }
    pub fn read_u8_static(&self) -> Result<u8, ()> {
        Ok(self.read_n_bytes_static(1)?[0])
    }

    pub fn read_u16(&mut self) -> Result<u16, ()> {
        Ok(BigEndian::read_u16(self.read_n_bytes(2)?))
    }
    pub fn read_u16_static(&self) -> Result<u16, ()> {
        Ok(BigEndian::read_u16(self.read_n_bytes_static(2)?))
    }

    pub fn read_u32(&mut self) -> Result<u32, ()> {
        Ok(BigEndian::read_u32(self.read_n_bytes(4)?))
    }
    pub fn read_u32_static(&self) -> Result<u32, ()> {
        Ok(BigEndian::read_u32(self.read_n_bytes_static(4)?))
    }

    pub fn read_u64(&mut self) -> Result<u64, ()> {
        Ok(BigEndian::read_u64(self.read_n_bytes(8)?))
    }
    pub fn read_u64_static(&self) -> Result<u64, ()> {
        Ok(BigEndian::read_u64(self.read_n_bytes_static(8)?))
    }

    pub fn read_f32(&mut self) -> Result<f32, ()> {
        Ok(BigEndian::read_f32(self.read_n_bytes(4)?))
    }
    pub fn read_f32_static(&self) -> Result<f32, ()> {
        Ok(BigEndian::read_f32(self.read_n_bytes_static(4)?))
    }

    pub fn read_bool(&mut self) -> Result<bool, ()> {
        Ok(self.read_u8()? != 0)
    }
    pub fn read_bool_static(&self) -> Result<bool, ()> {
        Ok(self.read_u8_static()? != 0)
    }

    /**
     * Reads string length as the first two bytes from the head of the buffer, then reads the string
     */
    pub fn read_string(&mut self) -> Result<String, ()> {
        let length = self.read_u16()?;
        let byte_string = self.read_n_bytes(length as usize)?;
        String::from_utf8(byte_string.to_vec()).or(Err(()))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    pub fn rem_to_vec(&self) -> Vec<u8> {
        self.bytes[self.index..].into()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    pub fn rem_len(&self) -> usize {
        self.bytes.len() - self.index
    }
}