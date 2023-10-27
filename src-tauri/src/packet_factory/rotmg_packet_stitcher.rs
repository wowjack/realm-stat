use std::collections::VecDeque;

use byteorder::ByteOrder;

use super::byte_buffer::ByteBuffer;



pub struct RotmgPacketStitcher {
    iqueue: VecDeque<u8>,
}
impl RotmgPacketStitcher {
    pub fn new() -> Self {
        Self { iqueue: VecDeque::new() }
    }

    pub fn insert_packet(&mut self, data: &[u8]) -> Vec<StitchedPacket> {
        self.iqueue.extend(data.iter());
        
        let mut out = Vec::with_capacity(1);

        loop {
            if self.iqueue.len() < 4 { return out }

            let next_packet_len = byteorder::BigEndian::read_u32(&[self.iqueue[0], self.iqueue[1], self.iqueue[2], self.iqueue[3]]);

            //return if the input queue isn't long enough to create another packet
            if self.iqueue.len() < next_packet_len as usize { return out }

            //Create a StitchedPacket and push it to the output queue
            out.push(StitchedPacket::new(self.iqueue.drain(0..next_packet_len as usize).collect()));
        }
    }

    pub fn reset(&mut self) {
        self.iqueue.clear();
    }
}

#[derive(Clone, Debug)]
pub struct StitchedPacket {
    pub type_num: u8,
    pub buffer: ByteBuffer
}
impl StitchedPacket {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            type_num: data[4],
            buffer: ByteBuffer::new(&data[5..])
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}