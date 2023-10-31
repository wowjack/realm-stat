use std::collections::VecDeque;

use crate::rc4::Rc4;

use super::{rotmg_packet_stitcher::StitchedPacket, rotmg_packet_decryptor::DecryptedTickFrame};


/// Group encrypted packets together into a frame terminating in a tick packet \
pub struct TickFrameConstructor {
    iqueue: VecDeque<StitchedPacket>,
}
impl TickFrameConstructor {
    pub fn new() -> Self {
        Self { iqueue: VecDeque::with_capacity(10) }
    }

    pub fn insert_packet(&mut self, packet: StitchedPacket) -> Option<EncryptedTickFrame> {
        if packet.type_num == 10 { // received a tick packet
            return Some(EncryptedTickFrame {
                packets: self.iqueue.drain(0..self.iqueue.len()).collect(),
                terminating_tick: packet
            });
        } else {
            self.iqueue.push_back(packet);
            return None;
        }
    }

    pub fn reset(&mut self) {
        self.iqueue.clear();
    }
}

#[derive(Clone)]
pub struct EncryptedTickFrame {
    pub packets: Vec<StitchedPacket>,
    pub terminating_tick: StitchedPacket,
}
impl EncryptedTickFrame {
    /// Returns the total size in bytes of all the packet payloads in the frame not including the terminating tick
    pub fn payload_len(&self) -> usize {
        self.packets.iter().fold(0, |acc, e| acc + e.len())
    }
    pub fn total_len(&self) -> usize {
        self.payload_len() + self.terminating_tick.len()
    }

    pub fn decrypt(&self, cipher: &mut Rc4) -> DecryptedTickFrame {
        let mut packets = self.packets.clone();
        packets.iter_mut().for_each(|p| {
            p.decrypt(cipher);
            if p.type_num == 45 {
                log::debug!("reset packet");
                cipher.reset();
            }
        });
        let mut terminating_tick = self.terminating_tick.clone();
        terminating_tick.decrypt(cipher);
        DecryptedTickFrame { packets, terminating_tick }
    }

    
}