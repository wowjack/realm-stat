pub mod rotmg_packet;
pub mod byte_buffer;
pub mod data_types;
mod rotmg_packet_constructor;
mod rotmg_packet_stitcher;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use etherparse::SlicedPacket;
use self::rotmg_packet::RotmgPacket;
use self::rotmg_packet_constructor::RotmgPacketConstructor;
use self::rotmg_packet_stitcher::RotmgPacketStitcher;




/// Stitches together, validates, and decrypts packets
pub struct RotmgPacketFactory {
    /// Object to get raw packet payloads and stitch them together into encrypted ROTMG packet data chunks
    pub stitcher: RotmgPacketStitcher,

    /// Object to receive encrypted ROTMG packet data chunks, decrypt them, then perform a sanity check
    pub constructor: RotmgPacketConstructor,

    /// Output queue of fully decrypted and constructed ROTMG packets
    ///* Wrapped in an arc and mutex since the constructor's pipe closure needs to capture it to send packets here
    oqueue: Arc<Mutex<VecDeque<RotmgPacket>>>
}
impl RotmgPacketFactory {
    pub fn new() -> Self {
        let stitcher = RotmgPacketStitcher::new();
        let oqueue = Arc::new(Mutex::new(VecDeque::new()));

        let pipe_oqueue = oqueue.clone();
        let constructor = RotmgPacketConstructor::new(Box::new(move |packets: Vec<RotmgPacket>| pipe_oqueue.lock().unwrap().extend(packets.into_iter())));

        Self { stitcher, constructor, oqueue }
    }

    /// Hand a sliced packet to the factory for processing
    pub fn insert_packet(&mut self, packet: SlicedPacket) {
        // Send packet to the stitcher, then send resulting stitched packets to the constructor
        // The constructor will get the fully processed packets to the oqueue on its own time
        self.stitcher
            .insert_packet(packet.payload)
            .into_iter()
            .for_each(|sp| self.constructor.insert_packet(sp));
    }


    /**
     * Get a rotmg packet from the head of the output queue
     */
    pub fn get_packet(&mut self) -> Option<RotmgPacket> {
        self.oqueue.lock().unwrap().pop_front()
    }

    pub fn get_packets(&mut self) -> Vec<RotmgPacket> {
        let mut oqueue = self.oqueue.lock().unwrap();
        let len = oqueue.len();
        oqueue.drain(0..len).collect()
    }

    pub fn reset(&mut self) {
        self.stitcher.reset();
        self.constructor.reset();
    }

    
}





