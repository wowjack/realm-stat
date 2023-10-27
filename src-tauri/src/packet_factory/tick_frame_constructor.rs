use std::{collections::VecDeque, sync::Arc};

use super::rotmg_packet_stitcher::StitchedPacket;


type TickFramePipe = Box<dyn Fn(TickFrame) + Send + Sync + 'static>;

/// Group encrypted packets together into a frame terminating in a tick packet
pub struct TickFrameConstructor {
    iqueue: VecDeque<StitchedPacket>,
    pipe: TickFramePipe,
}
impl TickFrameConstructor {
    pub fn new(pipe: TickFramePipe) -> Self {
        Self {
            iqueue: VecDeque::with_capacity(10),
            pipe,
        }
    }

    pub fn insert_packet(&mut self, packet: StitchedPacket) {
        if packet.type_num == 10 { // received a tick packet
            (self.pipe)(TickFrame {
                packets: self.iqueue.drain(0..self.iqueue.len()).collect(),
                terminating_tick: packet
            });
        } else {
            self.iqueue.push_back(packet);
        }
    }

    pub fn reset(&mut self) {
        self.iqueue.clear();
    }
}


pub struct TickFrame {
    pub packets: Vec<StitchedPacket>,
    pub terminating_tick: StitchedPacket,
}
impl TickFrame {
    /// Returns the total size in bytes of all the packet payloads in the frame not including the terminating tick
    pub fn payload_len(&self) -> usize {
        self.packets.iter().fold(0, |acc, e| acc + e.len())
    }
}