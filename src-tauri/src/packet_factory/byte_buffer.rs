#![allow(unused)]
use byteorder::{BigEndian, ByteOrder};



#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ByteBuffer {
    pub bytes: Vec<u8>,
    pub index: usize,
}
impl ByteBuffer {
    /// Construct a new ByteBuffer from a vector of bytes.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, index: 0 }
    }

    /// Reset the current index of the buffer to 0.
    pub fn reset(mut self) -> Self {
        self.index = 0;
        return self
    }

    /// Return the next n bytes from the buffer as a slice and move the current index.
    /// 
    /// Return an error if there are not n bytes left.
    pub fn read_n_bytes(&mut self, n: usize) -> Result<&[u8], ()> {
        self.index += n;
        if self.index > self.bytes.len() {
            self.index -= n;
            return Err(())
        }
        return Ok(&self.bytes[self.index-n..self.index])
    }
    /// Return the next n bytes from the buffer as a slice without moving the current index.
    /// 
    /// Return an error if there are not n bytes left.
    pub fn read_n_bytes_static(&self, n: usize) -> Result<&[u8], ()> {
        if self.index + n > self.bytes.len() {
            return Err(())
        }
        return Ok(&self.bytes[self.index..self.index+n])
    }

    /// Get the next byte from the buffer and move the current index.
    /// 
    /// Return an error if there are no bytes left.
    pub fn read_u8(&mut self) -> Result<u8, ()> {
        Ok(self.read_n_bytes(1)?[0])
    }
    /// Get the next byte from the buffer without moving the current index.
    /// 
    /// Return an error if there are no bytes left.
    pub fn read_u8_static(&self) -> Result<u8, ()> {
        Ok(self.read_n_bytes_static(1)?[0])
    }

    /// Get the next 2 bytes from the buffer as a u16 and move the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_u16(&mut self) -> Result<u16, ()> {
        Ok(BigEndian::read_u16(self.read_n_bytes(2)?))
    }
    /// Get the next 2 bytes from the buffer as a u16 without moving the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_u16_static(&self) -> Result<u16, ()> {
        Ok(BigEndian::read_u16(self.read_n_bytes_static(2)?))
    }

    /// Get the next 4 bytes from the buffer as a u32 and move the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_u32(&mut self) -> Result<u32, ()> {
        Ok(BigEndian::read_u32(self.read_n_bytes(4)?))
    }
    /// Get the next 4 bytes from the buffer as a u32 without moving the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_u32_static(&self) -> Result<u32, ()> {
        Ok(BigEndian::read_u32(self.read_n_bytes_static(4)?))
    }

    /// Get the next 8 bytes from the buffer as a u64 and move the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_u64(&mut self) -> Result<u64, ()> {
        Ok(BigEndian::read_u64(self.read_n_bytes(8)?))
    }
    /// Get the next 8 bytes from the buffer as a u64 without moving the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_u64_static(&self) -> Result<u64, ()> {
        Ok(BigEndian::read_u64(self.read_n_bytes_static(8)?))
    }

    /// Get the next 4 bytes from the buffer as a f32 and move the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_f32(&mut self) -> Result<f32, ()> {
        Ok(BigEndian::read_f32(self.read_n_bytes(4)?))
    }
    /// Get the next 4 bytes from the buffer as a f32 without moving the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_f32_static(&self) -> Result<f32, ()> {
        Ok(BigEndian::read_f32(self.read_n_bytes_static(4)?))
    }

    /// Get the next byte from the buffer as a bool and move the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_bool(&mut self) -> Result<bool, ()> {
        Ok(self.read_u8()? != 0)
    }
    /// Get the next byte from the buffer as a bool without the current index.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_bool_static(&self) -> Result<bool, ()> {
        Ok(self.read_u8_static()? != 0)
    }

    /// Get a utf8 encoded string from the buffer and move the current index to the end of the string.
    /// 
    /// The u16 at the current index denotes the length of the string.
    /// 
    /// Return an error if there are not enough bytes left.
    pub fn read_string(&mut self) -> Result<String, ()> {
        let length = self.read_u16()?;
        let byte_string = self.read_n_bytes(length as usize)?;
        String::from_utf8(byte_string.to_vec()).or(Err(()))
    }

    /// Get the next 2 bytes from the buffer as a compressed i32 and move the current index to the end of the string.
    /// 
    /// Return and error if there are not enough bytes left.
    pub fn read_compressed_i32(&mut self) -> Result<i32, ()> {
        let mut ubyte = self.read_u8()? as i32;
        let is_negative = (ubyte & 64) != 0;
        let mut shift = 6u32;
        let mut value = (ubyte & 63) as i32;

        while (ubyte & 128) != 0 {
            ubyte = self.read_u8()? as i32;
            value |= (ubyte & 127) << shift;
            shift += 7;
        }
        return Ok(if is_negative {-1*value} else {value})
    }

    /// Get a vec of compresed i32s from the buffer and move the current index to the end of the vec.
    /// 
    /// The leading compressed i32 denotes the length of the vec.
    /// 
    /// Return and error if there are not enough bytes left.
    pub fn read_compressed_i32_vec(&mut self) -> Result<Vec<i32>, ()> {
        let mut ret = vec![];
        for _ in 0..self.read_compressed_i32()? {
            ret.push(self.read_compressed_i32()?);
        }
        Ok(ret)
    }

    /// Return the entire contents of the buffer as a vec of bytes.
    /// 
    /// Does not move the current index.
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    /// Return the remaining contents of the buffer after the current index as a vec of bytes.
    /// 
    /// Does not move the current index.
    pub fn rem_to_vec(&self) -> Vec<u8> {
        self.bytes[self.index..].into()
    }

    /// Get the length of the entire buffer.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    /// Get the length of the buffer after the current index.
    pub fn rem_len(&self) -> usize {
        self.bytes.len() - self.index
    }
}