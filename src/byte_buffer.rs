use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Clone)]
pub struct ByteBuffer {
    bytes: Vec<u8>,
    index: usize,
}
impl ByteBuffer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, index: 0 }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<&[u8], ()> {
        self.index += n;
        if self.index > self.bytes.len() {
            self.index -= n;
            return Err(())
        }
        return Ok(&self.bytes[self.index-n..self.index])
    }

    pub fn read_u8(&mut self) -> Result<u8, ()> {
        Ok(self.read_n_bytes(1)?[0])
    }

    pub fn read_u16(&mut self) -> Result<u16, ()> {
        Ok(BigEndian::read_u16(self.read_n_bytes(2)?))
    }

    pub fn read_u32(&mut self) -> Result<u32, ()> {
        Ok(BigEndian::read_u32(self.read_n_bytes(4)?))
    }

    pub fn read_u64(&mut self) -> Result<u64, ()> {
        Ok(BigEndian::read_u64(self.read_n_bytes(8)?))
    }

    pub fn read_f32(&mut self) -> Result<f32, ()> {
        Ok(BigEndian::read_f32(self.read_n_bytes(4)?))
    }

    pub fn read_bool(&mut self) -> Result<bool, ()> {
        Ok(self.read_u8()? != 0)
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
}