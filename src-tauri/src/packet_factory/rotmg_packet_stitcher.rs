use std::collections::VecDeque;

use byteorder::ByteOrder;

use super::byte_buffer::ByteBuffer;



pub struct RotmgPacketStitcher {
    iqueue: VecDeque<u8>,
    oqueue: VecDeque<StitchedPacket>,
}
impl RotmgPacketStitcher {
    pub fn new() -> Self {
        Self {
            iqueue: VecDeque::new(),
            oqueue: VecDeque::new(),
        }
    }

    pub fn insert_packet(&mut self, data: &[u8]) {
        self.iqueue.extend(data.iter());
        self.check_queue();
    }
    pub fn get_packet(&mut self) -> Option<StitchedPacket> {
        self.oqueue.pop_front()
    }

    /**
     * Checks if the queue is long enough to construct another packet
     */
    fn check_queue(&mut self) {
        loop {
            if self.iqueue.len() < 4 {
                return
            }

            let next_packet_len = byteorder::BigEndian::read_u32(&[self.iqueue[0], self.iqueue[1], self.iqueue[2], self.iqueue[3]]);

            //log::debug!("Expecting packet of size {next_packet_len}, {:?}", self.iqueue.as_slices());

            //return if the input queue isn't long enough to create another rotmg packet
            if self.iqueue.len() < next_packet_len as usize {
                return
            }

            //Create a StitchedPacket and push it to the output queue
            let mut application_data = ByteBuffer::new(self.iqueue.drain(0..next_packet_len as usize).collect());
            let _ = application_data.read_n_bytes(4);

            if let Ok(t) = application_data.read_u8() {
                self.oqueue.push_back(StitchedPacket { type_num: t, data: application_data });
            }
        }
    }

    pub fn reset(&mut self) {
        self.iqueue.clear();
        self.oqueue.clear();
    }
}

#[derive(Clone, Debug)]
pub struct StitchedPacket {
    pub type_num: u8,
    pub data: ByteBuffer
}