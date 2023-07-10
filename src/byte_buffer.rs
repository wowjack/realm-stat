use byteorder::{BigEndian, ByteOrder};

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

    pub fn read_n_bytes(&mut self, n: usize) -> Option<&[u8]> {
        self.index += n;
        if self.index > self.bytes.len() {
            self.index -= n;
            return None
        }
        return Some(&self.bytes[self.index-n..self.index])
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        match self.read_n_bytes(1) {
            None => return None,
            Some(b) => Some(b[0])
        }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        match self.read_n_bytes(2) {
            None => return None,
            Some(b) => Some(BigEndian::read_u16(b))
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        match self.read_n_bytes(4) {
            None => return None,
            Some(b) => Some(BigEndian::read_u32(b))
        }
    }

    pub fn read_f32(&mut self) -> Option<f32> {
        match self.read_n_bytes(4) {
            None => return None,
            Some(b) => Some(BigEndian::read_f32(b))
        }
    }

    pub fn read_string(&mut self) -> Option<String> {
        let length = match self.read_u16() {Some(n) => n, None => return None};
        let byte_string = match self.read_n_bytes(length as usize) {Some(b) => b, None => return None};
        return String::from_utf8(byte_string.to_vec()).ok()
    }
}